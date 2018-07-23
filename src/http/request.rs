use bytes::Bytes;
use std::collections::HashMap;

#[derive(Debug)]
pub enum QueryStringValue {
    Single(Bytes),
    Array(Vec<Bytes>)
}

#[derive(Debug)]
pub struct Request {
    method: Bytes,
    uri: Bytes,
    query: HashMap<Bytes,QueryStringValue>,
    headers: HashMap<Bytes,Bytes>,
    body: Bytes
}
impl Request {
    pub fn new () -> Self {
        Request {
            method: Bytes::new(),
            uri: Bytes::new(),
            query: HashMap::new(),
            headers: HashMap::new(),
            body: Bytes::new()
        }
    }

    pub fn add_method(&mut self, method: Bytes) -> &mut Self {
        self.method = method;
        self
    }

    pub fn add_uri(&mut self, uri: Bytes) -> &mut Self {
        self.uri = uri;
        self
    }

    pub fn add_query_param(&mut self, name: Bytes, value: Bytes) -> &mut Self {
        if name[name.len()-1] == b']' && name[name.len()-2] == b'[' {
            self.query.entry(name).and_modify(|v| {
                match v {
                    QueryStringValue::Array(ref mut vec) => vec.push(value.clone()),
                    _ => {}
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
}