use super::route::RouteAction;
use crate::acceptor::Acceptor;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use threadpool::ThreadPool;
use super::route::{Routes, RouterResult};
use crate::http::method::MethodKind;
use crate::http::{HttpReader, Event, RequestBuilder, Response};
use std::sync::Arc;

pub struct Router {
    inner: Arc<InnerRouter>
}

pub struct InnerRouter {
    pub get: Routes,
    pub post: Routes,
    pub put: Routes,
    pub delete: Routes,
}

#[allow(dead_code)]
impl Router {
    pub fn new() -> Self {
        Router{
            inner: Arc::new(InnerRouter {
                get: Routes::new(),
                post: Routes::new(),
                put: Routes::new(),
                delete: Routes::new(),
            })
        }
    }
    fn inner_mut(&mut self) -> Option<&mut InnerRouter> {
        Arc::get_mut(&mut self.inner)
    }
    pub fn get<F: RouteAction + Sized>(&mut self, uri: &'static [u8], cb: F) -> &mut Self {
        if let Some(inner) = self.inner_mut() {
            inner.get.set_route(uri, cb);
        }
        self
    }

    pub fn post<F: RouteAction + Sized>(&mut self, uri: &'static [u8], cb: F) -> &mut Self {
        if let Some(inner) = self.inner_mut() {
            inner.post.set_route(uri, cb);
        }
        self
    }

    pub fn put<F: RouteAction + Sized>(&mut self, uri: &'static [u8], cb: F) -> &mut Self {
        if let Some(inner) = self.inner_mut() {
            inner.put.set_route(uri, cb);
        }
        self
    }

    pub fn delete<F: RouteAction + Sized>(&mut self, uri: &'static[u8], cb: F) -> &mut Self {
        if let Some(inner) = self.inner_mut() {
            inner.delete.set_route(uri, cb);
        }
        self
    }

    pub fn bind(&mut self, host: &str, port: u32) {
        let bind_on = format!("{}:{}", host, port);

        let mut acceptor = Acceptor::new(7, self.inner.clone());
        acceptor.listen(host, port);
    }
}

fn parse_http_request(mut stream: TcpStream) -> Result<(RequestBuilder, TcpStream),u8> {
    let mut request = RequestBuilder::new();
    println!("ttl => {}",stream.ttl().unwrap_or(0));
    {
        use self::Event::*;
        let mut buffer = [0u8; 8*1024];
        let reader = HttpReader::new(&mut stream, &mut buffer); 
        
        for event in reader {
            match event {
                Header(name,value) => { request.add_header(name, value); },
                Method(name) => { request.add_method(name); },
                Uri(uri) => { request.add_uri(uri); },
                Version(version) => { request.add_version(version); },
                QueryStringParam(name,value) => { request.add_query_param(name, value); },
                Body(body) => { request.add_body(body); },
                Err(errorno) => { 
                    println!("error: {}",errorno);
                    return Result::Err(errorno);
                },
                _ => ()
            };
        }
    }

    Ok((request, stream))
}

fn run_request_callback(request: RequestBuilder, inner: Arc<InnerRouter>) -> RouterResult {
    let response = Response::new();
    match request.method() {
        MethodKind::Get => inner.get.run_callback(request, response),
        MethodKind::Post => inner.post.run_callback(request, response),
        MethodKind::Put => inner.put.run_callback(request, response),
        MethodKind::Delete => inner.delete.run_callback(request, response),
        _ => Err(())
    }
}