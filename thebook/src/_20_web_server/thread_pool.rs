use std::sync::{mpsc, Arc, Mutex};
use std::thread;

type Job = Box<dyn FnOnce() + Send + 'static>;

enum Message {
    NewJob(Job),
    Terminate,
}

pub struct ThreadPool {
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
    pub fn new(size: usize) -> ThreadPool {
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
