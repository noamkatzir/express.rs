use express::server::router::Router;
use express::server::method::RequestMethod;
use express::server::protocol::Protocol;
use express::server::handler::{RequestBuilder, ResponseBuilder};
use std::sync::{Arc,Mutex};
use std::collections::HashMap;
use bytes::{Bytes, BytesMut, BufMut};
mod common;

type CallMap = Arc<Mutex<HashMap<u64,bool>>>;
#[test]
fn single_route_test() {
    let validation_key = Arc::new(common::rand_u64());
    let call_map: CallMap = Arc::new(Mutex::new(HashMap::new()));
    let uri = b"/path/to/action";
    let mut router = Router::new();
    given_routing_for(&mut router, uri, validation_key.clone(), call_map.clone());
    let mut request_builder = RequestBuilder::new(RequestMethod::Get, Protocol::HTTP1, Bytes::from(&uri[..]));
    let result = router.call(request_builder /*&RequestMethod::Get, &Bytes::from(&uri[..])*/);
    validate_response(&call_map, validation_key);
}

#[test]
fn multiple_route_test() {
    // let routing_response1 = Arc::new(common::rand_string(10));
    let routing1 = b"/path/to/action1";
    // let routing_response2 = Arc::new(common::rand_string(10));
    let routing2 = b"/path/to/action2";

    let validation_key1 = Arc::new(common::rand_u64());
    let validation_key2 = Arc::new(common::rand_u64());
    let call_map: CallMap = Arc::new(Mutex::new(HashMap::new()));

    let mut router = Router::new();
    given_routing_for(&mut router, routing1, validation_key1.clone(), call_map.clone());
    given_routing_for(&mut router, routing2, validation_key2.clone(), call_map.clone());
    
    let mut request_builder1 = RequestBuilder::new(RequestMethod::Get, Protocol::HTTP1, Bytes::from(&routing1[..]));
    let result = router.call(request_builder1 /*&RequestMethod::Get, &Bytes::from(&routing1[..])*/);
    validate_response(&call_map, validation_key1);

    let mut request_builder2 = RequestBuilder::new(RequestMethod::Get, Protocol::HTTP1, Bytes::from(&routing2[..]));
    let result = router.call(request_builder2 /*&RequestMethod::Get, &Bytes::from(&routing2[..])*/);
    validate_response(&call_map, validation_key2);
}

fn given_routing_for(router: &mut Router, uri: &'static [u8], validation_key: Arc<u64>, call_map: CallMap) {
    router.get(uri, move |_dummy| { 
        let respones_builder = ResponseBuilder::new();
        let mut map_mutator = call_map.lock().unwrap();
        (*map_mutator).insert(*validation_key,true);
        respones_builder
    });
}

fn validate_response(call_map: &CallMap, validation_key: Arc<u64>) {
    let map_getter = call_map.lock().unwrap();
    (*map_getter).get(&*validation_key).expect("result returned error");
}