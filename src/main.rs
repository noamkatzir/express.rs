extern crate threadpool;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
mod http_parser;
mod types;
use http_parser::{Request, QueryStringParam};
use types::HttpHeader::*;
use threadpool::ThreadPool;


fn handle_connection(mut stream: TcpStream) {
    let request = Request::parse(&mut stream).unwrap();
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

    // let mut buffer = [0u8; 80*1024];

    // stream.read(&mut buffer).unwrap();

    // println!("Request: {} END", String::from_utf8_lossy(&buffer[..]));
    
    let response = format!("HTTP/1.1 200 OK\r\n{}\r\n\r\n{}","Server:Noams","Hello World!");

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}


fn main() {
    let pool:ThreadPool = Default::default();
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    for stream in listener.incoming() {
        let mut stream = stream.unwrap();
        pool.execute(move || {
            handle_connection(stream);
        });
    }
}
