extern crate threadpool;
extern crate bytes;
extern crate express;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use express::http::{HttpReader, Event, Request, Response};
use threadpool::ThreadPool;
use Event::*;
use bytes::Bytes;
use express::router::router::Router;

// fn handle_connection(mut stream: TcpStream) {
//     let mut request = Request::new();

//     {
//         let mut buffer = [0u8; 80*1024];
//         let reader = HttpReader::new(&mut stream, &mut buffer); 

//         for event in reader {
//             match event {
//                 Header(name,value) => { request.add_header(name, value); },
//                 Method(name) => { request.add_method(name); },
//                 Uri(uri) => { request.add_uri(uri); },
//                 QueryStringParam(name,value) => { request.add_query_param(name, value); },
//                 Body(body) => { request.add_body(body); },
//                 Err => println!("error"),
//                 _ => {}
//             };
//         }
//     }
    
//     let mut response = Response::new();
//     let response = response.add_json_headers()
//                         .add_full_body(Bytes::from_static(b"{\"noam\":\"noam\"}"))
//                         .generate();
//     // println!("{:?}", String::from_utf8_lossy(&response[..]));
//     stream.write(&response[..]).unwrap();
//     stream.flush().unwrap();
// }


// fn main() {
//     let bind_on = "127.0.0.1:8080";
//     let pool:ThreadPool = Default::default();
//     let listener = TcpListener::bind(bind_on).unwrap();
//     println!("server started on {}", bind_on);
//     for stream in listener.incoming() {
//         let mut stream = stream.unwrap();
//         pool.execute(|| {
//             handle_connection(stream);
//         });
//     }
// }

fn page (req: Request, mut res: Response) -> Response {
    res.add_json_headers();
    res.add_full_body(Bytes::from_static(b"{\"key\":\"value\"}"));
    res
}

fn home (req: Request, mut res: Response) -> Response {
    res.add_json_headers();
    res.add_full_body(Bytes::from_static(b"{\"home\":\"value\"}"));
    res
}

fn main() {
    let mut router = Router::new();

    router.get(b"/home/noam/page", |req: Request, mut res: Response| {
        res.add_json_headers();
        res.add_full_body(Bytes::from_static(b"{\"key\":\"value1\"}"));
        res
    });

    router.get(b"/home/noam2/page2", |req: Request, mut res: Response| {
        res.add_json_headers();
        res.add_full_body(Bytes::from_static(b"{\"key\":\"value2\"}"));
        res
    });


    // router.get(b"/home/noam/page", page);

    // router.get(b"/", home);

    loop {
        router.bind("localhost", 8080);
    }
}