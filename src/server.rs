
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use bytes::Bytes;

pub mod router;
pub mod method;
pub mod handler;
use router::Router;
use method::RequestMethod;

#[allow(dead_code)]
pub struct Server {
    host: String,
    port: u32,
    router: Router
}

impl Server {
    pub fn new(host: &str, port: u32, router: Router) -> Self {
        Server {
            host: String::from(host),
            port,
            router
        }
    }

    pub fn bind(&self) -> std::io::Result<()> {
        let listener = TcpListener::bind(format!("{}:{}", self.host, self.port))?;

        // accept connections and process them serially
        for stream in listener.incoming() {
            self.handle_client(stream?);
        }
        Ok(())
    }

    fn handle_client(&self, mut stream: TcpStream) -> std::io::Result<()> {
        let mut buffer = [0u8;1024];
        stream.read(&mut buffer);

        let route = parse_buffer(&buffer);
        let result = self.router.call(&RequestMethod::Get, &route)?;
        stream.write_fmt(format_args!("{}", result));
        Ok(())
    }
}

fn parse_buffer(buffer: &[u8]) -> Bytes {
    for (index, value) in buffer.iter().enumerate() {
        if value == &b'\0' {
            return Bytes::from(&buffer[0..index])
        }
    }
    Bytes::from(&buffer[..])
}