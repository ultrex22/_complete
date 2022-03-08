use std::thread;

pub struct ThreadPool {
    threads: Vec<thread::JoinHandle<()>>,

}

impl ThreadPool {
    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// #panics
    ///
    /// The 'new' fn will panic if the size is zero.
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);
        let mut threads = Vec::with_capacity(size);
        for _ in 0..size {
            // create threads
        }
        ThreadPool { threads }
    }

    pub fn execute<F>(&self, f: F)
        where
            F: FnOnce() + Send + 'static
    {}
}