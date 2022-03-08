use std::sync::mpsc;
use std::thread;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,

}

impl ThreadPool {
    /// Create a new ThreadPool.
    /// The size is the number of threads in the pool.
    /// #panics
    /// The 'new' fn will panic if the size is zero.
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, receiver));
        }
        ThreadPool{workers,sender}
    }

    pub fn execute<F>(&self, f: F)
        where
            F: FnOnce() + Send + 'static
    {

    }
}

/// worker exists so that we dont simply create a pool of threads
/// and use then right way. instead we have the worker struct.
struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

struct Job;

impl Worker {
    fn new(id:usize, receiver:mpsc::Receiver<Job>) -> Worker {
        let thread = thread::spawn(|| {
            receiver;
        });
        Worker{id, thread}
    }
}