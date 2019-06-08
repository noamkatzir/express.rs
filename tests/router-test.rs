use express::server::handler::Request;
use std::sync::mpsc::Receiver;
use express::server::router::Router;
use express::server::method::RequestMethod;
use express::server::protocol::Protocol;
use express::server::handler::{RequestBuilder, ResponseBuilder};
use std::sync::{Arc,Mutex};
use std::collections::HashMap;
use std::sync::mpsc::channel;
use bytes::{Bytes, BytesMut, BufMut};
mod common;

type CallMap = Arc<Mutex<HashMap<u64,bool>>>;
#[test]
fn single_route_test() {
    let uri = b"/path/to/action";
    let mut router = Router::new();
    let receiver = given_routing_for(&mut router, uri);
    let request_builder = RequestBuilder::new(RequestMethod::Get, Protocol::HTTP1, Bytes::from(&uri[..]));
    let result = router.call(request_builder);
    receiver.try_recv().expect("result returned error");
}

#[test]
fn multiple_route_test() {
    let routing1 = b"/path/to/action1";
    let routing2 = b"/path/to/action2";
    
    let mut router = Router::new();
    let receiver1 = given_routing_for(&mut router, routing1);
    let receiver2 = given_routing_for(&mut router, routing2);
    
    let request_builder1 = RequestBuilder::new(RequestMethod::Get, Protocol::HTTP1, Bytes::from(&routing1[..]));
    router.call(request_builder1);
    receiver1.try_recv().expect("result returned error");

    let request_builder2 = RequestBuilder::new(RequestMethod::Get, Protocol::HTTP1, Bytes::from(&routing2[..]));
    let result = router.call(request_builder2);
    receiver2.try_recv().expect("result returned error");
}

fn given_routing_for(router: &mut Router, uri: &'static [u8]) -> Receiver<Request> {
    let (transmitter, receiver) = channel();
    router.get(uri, move |request| { 
        let respones_builder = ResponseBuilder::new(Bytes::from(common::rand_string(10)));
        transmitter.send(request).unwrap();
        respones_builder
    });
    receiver
}