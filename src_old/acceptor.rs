use std::thread;
use std::time::Instant;
use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::net::{TcpListener, TcpStream};
use std::io::prelude::*;
use crate::router::route::RouterResult;
use crate::router::router::InnerRouter;
use crate::http::method::MethodKind;
use crate::http::{HttpReader, Event, RequestBuilder, Response};

enum Message {
    Socket(TcpStream, Instant),
    Terminate,
}

pub struct Acceptor {
    workers: Vec<ConnectionHandler>,
    sender: mpsc::Sender<Message>,
    counter: Arc<AtomicUsize>,
    inner: Arc<InnerRouter>
}

impl Acceptor {
    pub fn new(size: usize, inner: Arc<InnerRouter>) -> Self {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        let counter = Arc::new(AtomicUsize::new(0));

        for id in 0..size {
            
            workers.push(ConnectionHandler::new(id, sender.clone(), receiver.clone(), counter.clone(), inner.clone()));
        }

        Acceptor {
            workers,
            sender,
            counter,
            inner
        }
    }

    pub fn listen(&mut self, host: &str, port: u32) {
        let bind_on = format!("{}:{}", host, port);
        let listener = TcpListener::bind(bind_on.clone()).unwrap();

        for stream in listener.incoming() {
            let stream = stream.unwrap();
            loop {
                let result = self.counter.load(Ordering::Acquire);
                if result < 1900 {
                    break;
                }
            }
            self.counter.fetch_add(1, Ordering::Release);
            self.sender.send(Message::Socket(stream, Instant::now())).unwrap();
        }
    }
}

impl Drop for Acceptor {
    fn drop(&mut self) {
        println!("Sending terminate message to all workers.");

        for _ in &mut self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }

        println!("Shutting down all workers.");

        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);

            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

// impl Default for Acceptor {
//     fn default() -> Self {
//         Acceptor::new(num_cpus::get()-1)
//     }
// }

struct ConnectionHandler {
    id: usize,
    thread: Option<thread::JoinHandle<()>>
}

impl ConnectionHandler {
    fn new( id: usize, sender: mpsc::Sender<Message>, receiver: Arc<Mutex<mpsc::Receiver<Message>>>, 
            counter: Arc<AtomicUsize>, inner: Arc<InnerRouter>) -> ConnectionHandler {

        let thread = thread::spawn(move ||{
            loop {
                let message = receiver.lock().unwrap().recv().unwrap();
                match message {
                    Message::Socket(mut stream, time) => {
                        if time.elapsed().as_secs() < 10 {
                            let has_keep_alive = handle_request(&mut stream, &inner);

                            let result = counter.load(Ordering::Acquire);
                            if has_keep_alive && result < 1900 {
                                sender.send(Message::Socket(stream, time)).unwrap();
                            } else {
                                counter.fetch_sub(1, Ordering::Release);
                            }
                            
                        }
                    },
                    Message::Terminate => {
                        println!("ConnectionHandler {} was told to terminate.", id);
                        break;
                    },
                }
            }
        });

        ConnectionHandler {
            id,
            thread: Some(thread),
        }
    }
}

fn parse_http_request(stream: &mut TcpStream) -> Result<(RequestBuilder),u8> {
    let mut request = RequestBuilder::new();

    {
        use self::Event::*;
        let mut buffer = [0u8; 8*1024];
        let reader = HttpReader::new(stream, &mut buffer); 
        
        for event in reader {
            match event {
                Header(name,value) => { request.add_header(name, value); },
                Method(name) => { request.add_method(name); },
                Uri(uri) => { request.add_uri(uri); },
                Version(version) => { request.add_version(version); },
                QueryStringParam(name,value) => { request.add_query_param(name, value); },
                Body(body) => { request.add_body(body); },
                Err(errorno) => { 
                    println!("error: {}",errorno);
                    return Result::Err(errorno);
                },
                _ => ()
            };
        }
    }

    Ok(request)
}

fn run_request_callback(request: RequestBuilder, inner: &Arc<InnerRouter>) -> RouterResult {
    let response = Response::new();
    match request.method() {
        MethodKind::Get => inner.get.run_callback(request, response),
        MethodKind::Post => inner.post.run_callback(request, response),
        MethodKind::Put => inner.put.run_callback(request, response),
        MethodKind::Delete => inner.delete.run_callback(request, response),
        _ => Err(())
    }
}

fn handle_request(stream: &mut TcpStream, inner: &Arc<InnerRouter>) -> bool {
    if let Ok(request) = parse_http_request(stream) {
        let has_keep_alive = request.has_keep_alive();
        let result = run_request_callback(request, inner);
        {
            match result {
                Ok(response) => {
                    let response = response.generate();
                    stream.write(&response[..]).unwrap();
                    stream.flush().unwrap();
                },
                _ => { println!("routing error"); }
            }
        }

        has_keep_alive
    } else {
        false
    }
}