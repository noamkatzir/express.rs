use express::client::Client;
use express::server::router::Router;
use std::sync::Arc;
// use rand::*;
mod common;

#[test]
fn test_simple_server() {
    let body = Arc::new(common::rand_string(10));
    let other_body = body.clone();
    let mut router = Router::new();
    router.get(b"/path/to/action", move |_dummy| { (*other_body).clone() });

    common::given_server_started_with("localhost",8080,router);



    let client = Client::new("localhost",8080);
    match client.getRequest(b"/path/to/action") {
        Ok(result) => assert_eq!(result.body(), *body, "actual {} expected {}", result.body(), *body),
        Err(_) => assert!(false, "result returned error")
    }
}