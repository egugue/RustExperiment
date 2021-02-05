use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{mpsc, Arc, Mutex};
use std::time::Duration;
use std::{fs, thread};

pub fn main() {
    let thread_pool = ThreadPool::new(4);
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    println!("Started");

    // take only 2 requests to demonstrate graceful shutdown.
    for stream in listener.incoming().take(2) {
        let stream = stream.unwrap();
        thread_pool.execute(move || {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    // println!("\n{}\n", String::from_utf8_lossy(&buffer));

    let get_index = b"GET / HTTP/1.1\r\n";
    let get_sleep = b"GET /sleep HTTP/1.1\r\n";
    let (status_line, filename) = if buffer.starts_with(get_index) {
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else if buffer.starts_with(get_sleep) {
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };

    let contents = fs::read_to_string(format!("../officialbook/{}", filename)).unwrap();
    let response = format!("{}{}", status_line, contents);
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

type Job = Box<dyn FnOnce() + Send + 'static>;

enum Message {
    NewJob(Job),
    Terminate,
}

struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}

impl ThreadPool {
    /// Create a new ThreadPool
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    /// The function will panic if size is zero.
    fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);
        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool { workers, sender }
    }

    /// FnOnce trait bound: the closure will be executed once in a thread.
    /// Send  trait bound: the closure should transfer one thread to another thread.
    /// static lifetime bound: we don't know how long the closure will execute.
    ///
    /// () after FnOnce means the closure takes no parameters and returns the unit type.
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.send(Message::NewJob(job)).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        // Need to separate for-loop to prevent dead-locks
        // In a situation that worker1 is handling request and worker2 took a Terminate message,
        // ThreadPool will try to call worker1.thread.join but it never succeed
        // because worker1 didn't yet receive a Terminate message.
        //
        // for worker in &mut self.workers {
        //     self.sender.send(Message::Terminate).unwrap();
        //     if let Some(thread) = worker.thread.take() {
        //         thread.join().unwrap();
        //     }
        // }

        println!("Sending terminate message to all workers.");
        for _ in &self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }

        println!("Shutting down all workers.");

        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);
            if let Some(thread) = worker.thread.take() {
                // Worker.thread needs to be wrapped in Option because join() needs to `mut self`
                thread.join().unwrap();
            }
        }

        println!("Finish Shutting down all workers.");
    }
}

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            println!("Worker {} waiting for another job.", id);
            match receiver.lock().unwrap().recv().unwrap() {
                Message::NewJob(job) => {
                    println!("Worker {} got a job; executing.", id);
                    job();
                    println!("Worker {} finish the job.", id);
                }
                Message::Terminate => {
                    println!("Terminate thread: {}", id);
                    break;
                }
            }
        });
        Worker {
            id,
            thread: Some(thread),
        }
    }
}
