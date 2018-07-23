extern crate threadpool;
extern crate bytes;
extern crate chrono;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
mod http;
use http::{HttpReader, Event, Request, Response};
use threadpool::ThreadPool;
use Event::*;
use bytes::Bytes;

fn handle_connection(mut stream: TcpStream) {
    let mut request = Request::new();

    {
        let mut buffer = [0u8; 80*1024];
        let reader = HttpReader::new(&mut stream, &mut buffer); 

        for event in reader {
            match event {
                Header(name,value) => { request.add_header(name, value); },
                Method(name) => { request.add_method(name); },
                Uri(uri) => { request.add_uri(uri); },
                QueryStringParam(name,value) => { request.add_query_param(name, value); },
                Body(body) => { request.add_body(body); },
                Err => println!("error"),
                _ => {}
            };
        }
    }
    
    let mut response = Response::new();
    let response = response.add_json_headers()
                        .add_body(Bytes::from_static(b"{\"noam\":\"noam\"}"))
                        .generate();
    println!("{:?}", String::from_utf8_lossy(&response[..]));
    stream.write(&response[..]).unwrap();
    stream.flush().unwrap();
}


fn main() {
    let pool:ThreadPool = Default::default();
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    for stream in listener.incoming() {
        let mut stream = stream.unwrap();
        pool.execute(|| {
            handle_connection(stream);
        });
    }
}
