use std::iter;
// use std::sync::Arc;
use rand::{Rng, thread_rng};
use rand::distributions::Alphanumeric;

use std::thread;
use express::server::Server;
use express::server::router::Router;
#[allow(dead_code)]
pub fn given_server_started_with(host: &'static str, port: u32, router: Router) {

    thread::spawn(move || {
        let server = Server::new(host, port, router);

        server.bind();
    });
}

pub fn rand_string(n: usize) -> String {
    let mut rng = thread_rng();
    iter::repeat(())
        .map(|()| rng.sample(Alphanumeric))
        .take(n)
        .collect()
}

pub fn rand_u64() -> u64 {
        let mut rng = thread_rng();
        rng.gen::<u64>()
}