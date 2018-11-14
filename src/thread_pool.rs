use std::thread;
use std::time;
use std::sync::mpsc;
use std::sync::mpsc::TryRecvError;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::atomic::{AtomicUsize, Ordering};
use num_cpus;

enum Message {
    NewJob(Job),
    Terminate,
}

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
    counter: Arc<AtomicUsize>
}

trait FnBox {
    fn call_box(self: Box<Self>);
}

impl<F: FnOnce()> FnBox for F {
    fn call_box(self: Box<F>) {
        (*self)()
    }
}

type Job = Box<FnBox + Send + 'static>;

impl ThreadPool {
    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero.
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        let counter = Arc::new(AtomicUsize::new(0));

        for id in 0..size {
            
            workers.push(Worker::new(id, receiver.clone(), counter.clone() ));
        }

        ThreadPool {
            workers,
            sender,
            counter
        }
    }

    pub fn execute<F>(&mut self, f: F)
        where
            F: FnOnce() + Send + 'static
    {
        // let mut gotToTop = false;
        loop {
            let result = self.counter.load(Ordering::Acquire);
            
            if result < 900 {
                break;
            }
            // if(!gotToTop) {
            //     println!("going to sleep! counter: {}", result);
            //     gotToTop = true;
            // }
            
        }
        
        let job = Box::new(f);
        self.sender.send(Message::NewJob(job)).unwrap();
        let old_count = self.counter.fetch_add(1, Ordering::Release);
        // if old_count as f32 > 900.0 * 0.75 {
        //     println!("going to sleep! counter: {}", old_count);
        //     thread::sleep(time::Duration::from_millis(50));
        // }
    }
}

impl Drop for ThreadPool {
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

impl Default for ThreadPool {
    fn default() -> Self {
        ThreadPool::new(num_cpus::get()-1)
    }
}

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>, counter: Arc<AtomicUsize>) ->
        Worker {

        let thread = thread::spawn(move ||{
            loop {
                let message = receiver.lock().unwrap().recv().unwrap();
                match message {
                    Message::NewJob(job) => {
                        // println!("Worker {} got a job; executing.", id);
                        job.call_box();
                        let result = counter.fetch_sub(1, Ordering::Release);
                        // if result >= 850 {
                        //     println!("counter: {}", result);
                        // }
                    },
                    Message::Terminate => {
                        println!("Worker {} was told to terminate.", id);
                        break;
                    },
                }
            }
        });

        Worker {
            id,
            thread: Some(thread),
        }
    }
}