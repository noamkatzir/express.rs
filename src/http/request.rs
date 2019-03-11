use bytes::{Bytes};
use std::collections::HashMap;
use super::method::MethodKind;
use super::protocol::Protocol;

#[derive(Debug)]
pub enum QueryStringValue {
    Single(Bytes),
    Array(Vec<Bytes>)
}

#[derive(Debug)]
pub struct RequestBuilder {
    method: MethodKind,
    uri: Bytes,
    version: Protocol,
    uri_params: HashMap<Bytes,Bytes>,
    query: HashMap<Bytes,QueryStringValue>,
    headers: HashMap<Bytes,Bytes>,
    body: Bytes
}
impl RequestBuilder {
    pub fn new () -> Self {
        RequestBuilder {
            method: MethodKind::Unknown,
            uri: Bytes::new(),
            query: HashMap::new(),
            headers: HashMap::new(),
            uri_params: HashMap::new(),
            body: Bytes::new(),
            version: Protocol::Unknown
        }
    }

    pub fn add_method(&mut self, method: Bytes) -> &mut Self {
        self.method = match &method[..] {
            b"GET" => MethodKind::Get,
            b"POST" => MethodKind::Post,
            b"PUT" => MethodKind::Put,
            b"DELETE" => MethodKind::Delete,
            _ => MethodKind::Unknown
        };
        self
    }

    pub fn add_uri(&mut self, uri: Bytes) -> &mut Self {
        self.uri = uri;
        self
    }

    pub fn add_version(&mut self, version: Bytes) -> &mut Self {
        self.version = match &version[..] {
            b"HTTP/1.1" => Protocol::HTTP11,
            b"HTTP/1.0" => Protocol::HTTP1,
            _ => Protocol::Unknown
        };
        self
    }

    pub fn add_uri_params(&mut self, uri_params: HashMap<Bytes,Bytes>) -> &mut Self {
        self.uri_params = uri_params;
        self
    }

    pub fn add_query_param(&mut self, name: Bytes, value: Bytes) -> &mut Self {
        if name[name.len()-1] == b']' && name[name.len()-2] == b'[' {
            self.query.entry(name).and_modify(|v| {
                match v {
                    QueryStringValue::Array(ref mut vec) => vec.push(value.clone()),
                    _ => ()
                }
            }).or_insert(QueryStringValue::Array(vec![value]));
        } else { 
            self.query.insert(name, QueryStringValue::Single(value));
        }

        self
    }

    pub fn add_header(&mut self, name: Bytes, value: Bytes) -> &mut Self {
        self.headers.insert(name, value);
        self
    }

    pub fn add_body(&mut self, body: Bytes) -> &mut Self {
        self.body = body;
        self
    }

    pub fn build(self) -> Request {
        Request {
            method: self.method,
            uri: self.uri,
            uri_params: self.uri_params,
            query: self.query,
            headers: self.headers,
            body: self.body
        }
    }

    pub fn method(&self) -> MethodKind { self.method.clone() }

    pub fn has_keep_alive(&self) -> bool {
        match self.version {
            Protocol::HTTP1 => {
                match self.headers.get(&Bytes::from_static(b"Connection")) {
                    Some(con_value) if &con_value[..] == b" keep-alive" => true,
                    _ => false
                }
            }
            Protocol::HTTP11 => {
                match self.headers.get(&Bytes::from_static(b"Connection")) {
                    Some(con_value) if &con_value[..] == b" close" => false,
                    _ => true
                }
            }
            _ => false
        }
        
    }

    pub fn uri(&self) -> Bytes {
        self.uri.clone()
    }
}

#[allow(dead_code)]
pub struct Request {
    method: MethodKind,
    uri: Bytes,
    uri_params: HashMap<Bytes,Bytes>,
    query: HashMap<Bytes,QueryStringValue>,
    headers: HashMap<Bytes,Bytes>,
    body: Bytes
}

impl Request {
    pub fn method(&self) -> MethodKind { self.method.clone() }

    pub fn headers(&self) -> HashMap<Bytes,Bytes> { self.headers.clone() }

}