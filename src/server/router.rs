use std::collections::HashMap;
use bytes::Bytes;

use super::handler::*;
use super::method::*;


pub struct Router {
    get: HashMap<Bytes, Box<RequestHandler>>
}

impl Router {
    pub fn new() -> Self {
        Router {
            get: HashMap::new()
        }
    }

    pub fn get<F: RequestHandler + Sized>(&mut self, uri: &'static [u8], callback: F)  {
        self.get.insert(Bytes::from(uri), Box::new(callback));
    }

    pub fn call(&self, method: &RequestMethod, uri: &Bytes) -> std::io::Result<String> {

        match method {
            RequestMethod::Get => match self.get.get(uri) {
                Some(ref handler) => Ok(handler.action(String::from(""))),
                None => Err(std::io::Error::from(std::io::ErrorKind::NotFound))
            },
            _ => Err(std::io::Error::from(std::io::ErrorKind::NotFound))
        }
    }
}
