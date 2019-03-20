use express::server::router::Router;
use express::server::method::RequestMethod;
use std::sync::Arc;
use bytes::{Bytes, BytesMut, BufMut};
mod common;
#[test]
fn single_route_test() {
    let body = Arc::new(common::rand_string(10));
    let uri = b"/path/to/action";
    let mut router = Router::new();
    let other_body = body.clone();
    router.get(uri, move |_dummy| { (*other_body).clone() });
    match router.call(&RequestMethod::Get, &Bytes::from(&uri[..])) {
        Ok(result) => assert_eq!(result,(*body).clone()),
        Err(_) => assert!(false, "result returned error")
    }
}