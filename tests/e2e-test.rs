use express::client::Client;
use express::server::router::Router;
use std::sync::Arc;
use express::server::handler::{RequestBuilder, ResponseBuilder};
use bytes::{Bytes, BytesMut, BufMut};
// use rand::*;
mod common;

#[test]
fn test_simple_server() {
    let body = Arc::new(common::rand_string(10));
    let other_body = body.clone();
    let mut router = Router::new();
    router.get(b"/path/to/action", move |_dummy| { 
        let x = (*other_body).clone();
        let responses_builder = ResponseBuilder::new(Bytes::from((*other_body).clone()));
        (*other_body).clone();
        responses_builder
        });

    common::given_server_started_with("localhost",8080,router);



    let client = Client::new("localhost",8080);
    match client.getRequest(b"/path/to/action") {
        Ok(result) => assert_eq!(result.body(), *body, "actual {} expected {}", result.body(), *body),
        Err(_) => assert!(false, "result returned error")
    }
}