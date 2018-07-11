use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
mod http_parser;
use http_parser::{Request, QueryStringParam /*, MethodKind, CacheControl, HttpVersion */};
use http_parser::HttpHeader::*;

fn handle_connection(stream: &mut TcpStream) {
    
    let request = Request::parse(stream).unwrap();
    match request.get_parsed_header("Host") {
        Some(Host {name,port }) => println!("host:{} port:{}",name,port),
        _ => println!("missing host")
    }
    println!("uri: {}",request.uri());
    match request.query("ggg") {
        Some(QueryStringParam::ArrayParam(values)) => {
            println!("ggg: {:?}",values)
        },
        _ => {}
    }
    
    let response = "HTTP/1.1 200 OK\r\n\r\n";

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}


fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    for stream in listener.incoming() {
        let mut stream = stream.unwrap();
        handle_connection(&mut stream);
    }
}
