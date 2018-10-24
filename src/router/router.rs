use router::route::RouteAction;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::time::{Duration, SystemTime};
use threadpool::ThreadPool;
use super::route::{Routes, RouterResult};
use std::thread;
use bytes::Bytes;
use http::method::MethodKind;
use http::{HttpReader, Event, RequestBuilder, Request, Response};
use std::sync::Arc;

pub struct Router {
    inner: Arc<InnerRouter>
}

pub struct InnerRouter {
    get: Routes,
    post: Routes,
    put: Routes,
    delete: Routes,
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
        let pool:ThreadPool = Default::default();
        let listener = TcpListener::bind(bind_on.clone()).unwrap();
        println!("server started on {}", bind_on);
        for stream in listener.incoming() {
            let mut stream = stream.unwrap();
            let inner = self.inner.clone();
            pool.execute(move || {
                let request_start_time = SystemTime::now();
                let (request, mut stream) = parse_http_request(stream);
                let response = Response::new();

                let mut result = match request.method() {
                    MethodKind::Get => inner.get.run_callback(request, response),
                    MethodKind::Post => inner.post.run_callback(request, response),
                    MethodKind::Put => inner.put.run_callback(request, response),
                    MethodKind::Delete => inner.delete.run_callback(request, response),
                    _ => Err(())
                };

                match result {
                    Ok(ref mut response) => {
                        let response = response.generate();
                        stream.write(&response[..]).unwrap();
                        stream.flush().unwrap();
                        println!("request time: {:?}", request_start_time.elapsed().unwrap());
                    },
                    _ => { }
                }
            });
        }
    }
}

fn parse_http_request(mut stream: TcpStream) -> (RequestBuilder, TcpStream) {
    let mut request = RequestBuilder::new();

    {
        use self::Event::*;
        let mut buffer = [0u8; 8*1024];
        let reader = HttpReader::new(&mut stream, &mut buffer); 
        
        for event in reader {
            match event {
                Header(name,value) => { request.add_header(name, value); },
                Method(name) => { request.add_method(name); },
                Uri(uri) => { request.add_uri(uri); },
                QueryStringParam(name,value) => { request.add_query_param(name, value); },
                Body(body) => { request.add_body(body); },
                Err => println!("error"),
                _ => {}
            };
        }
    }

    (request, stream)
}