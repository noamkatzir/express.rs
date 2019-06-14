#![allow(unused_must_use)]
use express::server::handler::Request;
use std::sync::mpsc::Receiver;
use express::server::router::Router;
use express::server::method::RequestMethod;
use express::server::protocol::Protocol;
use express::server::handler::{RequestBuilder, ResponseBuilder};
use std::sync::mpsc::channel;
use std::collections::HashMap;
use bytes::{Bytes, BytesMut, BufMut};
mod common;

#[test]
fn single_route_test() {
    let uri = b"/path/to/action";
    let mut router = Router::new();
    let receiver = given_routing_for(&mut router, uri);
    let mut request_builder = RequestBuilder::new();
    request_builder.set_method(RequestMethod::Get)
        .set_protocol(Protocol::HTTP1)
        .set_uri(Bytes::from(&uri[..]));
    router.call(request_builder);
    receiver.try_recv().expect("expected router handler wasn't triggered");
}

#[test]
fn multiple_route_test() {
    let routing1 = b"/path/to/action1";
    let routing2 = b"/path/to/action2";
    
    let mut router = Router::new();
    let receiver1 = given_routing_for(&mut router, routing1);
    let receiver2 = given_routing_for(&mut router, routing2);
    
    let mut request_builder1 = RequestBuilder::new();
    request_builder1.set_method(RequestMethod::Get)
        .set_protocol(Protocol::HTTP1)
        .set_uri(Bytes::from(&routing1[..]));
    router.call(request_builder1);
    receiver1.try_recv().expect("expected router handler wasn't triggered");

    let mut request_builder2 = RequestBuilder::new();
    request_builder2.set_method(RequestMethod::Get)
        .set_protocol(Protocol::HTTP1)
        .set_uri(Bytes::from(&routing2[..]));
    router.call(request_builder2);
    receiver2.try_recv().expect("expected router handler wasn't triggered");
}


#[test]
fn router_action_called_with_related_parameters_test() {
    let uri = b"/path/to/action";
    let mut router = Router::new();
    let receiver = given_routing_for(&mut router, uri);
    let headers = given_some_headers();
    let query_params = given_some_query_params();

    let mut request_builder = RequestBuilder::new();
    request_builder
    .set_method(RequestMethod::Get)
    .set_protocol(Protocol::HTTP1)
    .set_uri(Bytes::from(&uri[..]))
    .set_headers(headers.clone())
    .set_query_params(query_params.clone());

    router.call(request_builder);
    let request = receiver.try_recv().expect("expected router handler wasn't triggered");
    validate_request_with_headers(&request, &headers);
    validate_request_with_query_params(&request, &query_params);
}

fn given_routing_for(router: &mut Router, uri: &'static [u8]) -> Receiver<Request> {
    let (transmitter, receiver) = channel();
    router.get(uri, move |request| { 
        let responses_builder = ResponseBuilder::new(Bytes::from(common::rand_string(10)));
        transmitter.send(request).unwrap();
        responses_builder
    });
    receiver
}

fn given_some_headers() -> HashMap<Bytes, Bytes> {
    let mut query_params = HashMap::new();
    for _ in 0..5 {
        let key = Bytes::from(common::rand_string(10));
        let value = Bytes::from(common::rand_string(10));
        query_params.insert(key, value);
    }
    query_params
}

fn given_some_query_params() -> HashMap<Bytes, Bytes> {
    let mut query_params = HashMap::new();
    for _ in 0..5 {
        let key = Bytes::from(common::rand_string(10));
        let value = Bytes::from(common::rand_string(10));
        query_params.insert(key, value);
    }
    query_params
}

fn validate_request_with_headers(request: &Request, headers: &HashMap<Bytes, Bytes>) {
    for (expected_header_name, expected_header_value) in headers.into_iter() {
        let actual_header_value = request.get_header(expected_header_name);
        assert_eq!(actual_header_value, Some(expected_header_value), "headers: {:?} -> {:?} != {:?} -> {:?}", expected_header_name, expected_header_value, expected_header_name, actual_header_value);
    }
}

fn validate_request_with_query_params(request: &Request, query_params: &HashMap<Bytes, Bytes>) {
    for (expected_query_param_name, expected_query_param_value) in query_params.into_iter() {
        let actual_header_value = request.get_query_param(expected_query_param_name);
        assert_eq!(actual_header_value, Some(expected_query_param_value), "query params: {:?} -> {:?} != {:?} -> {:?}", expected_query_param_name, expected_query_param_value, expected_query_param_name, actual_header_value);
    }
}