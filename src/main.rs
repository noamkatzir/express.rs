extern crate threadpool;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
mod http_parser;
use http_parser::{Request, QueryStringParam};
use http_parser::HttpHeader::*;
use threadpool::ThreadPool;


fn handle_connection(stream: Box<TcpStream>) {
    let mut stream = *stream;
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
    
    let response = "HTTP/1.1 200 OK\r\n\r\n";

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}


fn main() {
    let pool = ThreadPool::new(4);
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    for stream in listener.incoming() {
        let mut stream = Box::new(stream.unwrap());
        pool.execute(move || {
            handle_connection(stream);
        });
    }
}
