use std::io::prelude::*;
use std::net::TcpStream;

#[allow(dead_code)]
pub struct Response {
    body: String
}
impl Response {
    pub fn body(&self) -> String {
        String::from(self.body.clone())
    }
}

pub struct Client {
    host: String,
    port: u32
}
impl Client {
    pub fn new(host: &str, port: u32) -> Self {
        Client {
            host: String::from(host),
            port
        }
    }

    pub fn getRequest(&self, url: &'static [u8]) -> std::io::Result<Response> {
        let mut stream = TcpStream::connect(format!("{}:{}", self.host, self.port))?;
        stream.write(url)?;
        let mut body = String::new();
        stream.read_to_string(&mut body)?;
        Ok(Response {
            body
        })
    }
}