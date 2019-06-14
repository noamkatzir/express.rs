use crate::server::status::Status;
use crate::server::protocol::Protocol;
use crate::server::method::RequestMethod;
use bytes::Bytes;
use std::collections::HashMap;
use std::net::TcpStream;
use std::io::prelude::*;
use std::io::Result;

pub trait RequestHandler: Send + 'static {
    fn action(&self,request: Request) -> ResponseBuilder;
}

impl<F> RequestHandler for F
where
    F: Send + 'static + Fn(Request) -> ResponseBuilder,
{
    fn action(&self,request: Request) -> ResponseBuilder {
        (*self)(request)
    }
}
impl RequestHandler for Box<dyn RequestHandler> {
    fn action(&self,request: Request) -> ResponseBuilder {
         (**self).action(request)
    }
}

pub struct RequestBuilder {
    method: RequestMethod,
    protocol: Protocol,
    uri: Bytes,
    headers: HashMap<Bytes, Bytes>,
    query_params: HashMap<Bytes, Bytes>,
    body: Option<Bytes>
}

impl RequestBuilder {
    pub fn new() -> Self {
        RequestBuilder { 
            method: RequestMethod::Unknown, 
            protocol: Protocol::UNKNOWN,
            uri: Bytes::new(),
            headers: HashMap::new(),
            query_params: HashMap::new(),
            body: None
        }
    }

    pub fn set_method(&mut self, method: RequestMethod) -> &mut Self {
        self.method = method;
        self
    }

    pub fn set_protocol(&mut self, protocol: Protocol) -> &mut Self {
        self.protocol = protocol;
        self
    }

    pub fn set_uri(&mut self, uri: Bytes) -> &mut Self {
        self.uri = uri;
        self
    }

    pub fn set_headers(&mut self, headers: HashMap<Bytes, Bytes>) -> &mut Self {
        self.headers = headers;
        self
    }

    pub fn set_query_params(&mut self, query_params: HashMap<Bytes, Bytes>) -> &mut Self {
        self.query_params = query_params;
        self
    }

    pub fn set_body(&mut self, body: Bytes) -> &mut Self {
        self.body = Some(body);
        self
    }
    
    pub fn build(self) -> Request {
        Request {
            method: RequestMethod::Get,
            protocol: Protocol::HTTP1,
            uri: self.uri,
            headers: self.headers,
            query_params: self.query_params,
            body: None
        }
    }
}

#[derive(Debug)]
pub struct Request {
    method: RequestMethod,
    protocol: Protocol,
    uri: Bytes,
    headers: HashMap<Bytes, Bytes>,
    query_params: HashMap<Bytes, Bytes>,
    body: Option<Bytes>
}

impl Request {
    pub fn get_method(&self) -> &RequestMethod {
        &self.method
    }

//     pub fn get_protocol(&self) -> &Protocol {
//         &self.protocol
//     }

    pub fn get_uri(&self) -> &Bytes {
        &self.uri
    }

    pub fn get_header(&self, header_name: &Bytes) -> Option<&Bytes> {
        self.headers.get(header_name)
    }

    pub fn get_query_param(&self, query_param_name: &Bytes) -> Option<&Bytes> {
        self.query_params.get(query_param_name)
    }

//     pub fn get_body(&self) -> Option<&Bytes> {
//         match self.body {
//             Some(ref data) => Some(data),
//             None => None
//         }
//     }

}

pub struct ResponseBuilder {
    protocol: Protocol,
    status: Status,
    headers: Vec<(Bytes,Bytes)>,
    body: Option<Bytes>
}

impl ResponseBuilder {
//     pub fn set_protocol(&mut self, protocol: Protocol) -> &mut Self {
//         self.protocol = protocol;
//         self
//     }

//     pub fn set_status(&mut self, status: Status) -> &mut Self {
//         self.status = status;
//         self
//     }

//     pub fn add_header(&mut self, )

    pub fn new(body: Bytes) -> Self {
        ResponseBuilder {
            protocol: Protocol::HTTP1,
            status: Status::OK,
            headers: Vec::new(),
            body: Some(body)
        }
    }

    pub fn send_response(self, stream: &mut TcpStream) -> Result<()> {
        let response = self.body.unwrap_or(Bytes::from_static(b""));
        
        stream.write(&response[..]).map(|_| {()})
    }
}