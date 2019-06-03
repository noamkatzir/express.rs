use std::collections::HashMap;
use bytes::Bytes;

use super::handler::*;
use super::method::*;


pub struct Router {
    get: HashMap<Bytes, Box<dyn RequestHandler>>
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

    pub fn call(&self, request_builder: RequestBuilder /*method: &RequestMethod, uri: &Bytes*/) -> std::io::Result<ResponseBuilder> {
        let request = request_builder.build();

        match request.get_method() {
            RequestMethod::Get => match self.get.get(request.get_uri()) {
                Some(ref handler) => {
                    Ok(handler.action(request))
                    },
                None => Err(std::io::Error::from(std::io::ErrorKind::NotFound))
            },
            _ => Err(std::io::Error::from(std::io::ErrorKind::NotFound))
        }
    }
}
