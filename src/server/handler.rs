use crate::server::status::Status;
use crate::server::protocol::Protocol;
use crate::server::method::RequestMethod;
use bytes::Bytes;
use std::collections::HashMap;
use std::net::TcpStream;
use std::io::prelude::*;
use std::io::Result;

pub trait RequestHandler: Send + Sync + 'static {
    fn action(&self,request: Request) -> ResponseBuilder;
}

impl<F> RequestHandler for F
where
    F: Send + Sync + 'static + Fn(Request) -> ResponseBuilder,
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
    body: Option<Bytes>
}

impl RequestBuilder {
    pub fn new(method: RequestMethod, protocol: Protocol, uri: Bytes) -> Self {
        RequestBuilder { 
            method, 
            protocol,
            uri,
            headers: HashMap::new(),
            body: None
        }
    }

    // pub fn get_method(&self) -> &RequestMethod {
    //     self.method.as_ref().unwrap()
    // }

    // pub fn get_uri(&self) -> &Bytes

    
    pub fn build(self) -> Request {
        Request {
            method: RequestMethod::Get,
            protocol: Protocol::HTTP1,
            uri: self.uri,
            headers: HashMap::new(),
            body: None
        }
    }
}

pub struct Request {
    method: RequestMethod,
    protocol: Protocol,
    uri: Bytes,
    headers: HashMap<Bytes, Bytes>,
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

//     pub fn get_header(&self, header_name: &Bytes) -> Option<&Bytes> {
//         self.headers.get(header_name)
//     }

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