use std::error::Error;
use std::sync::mpsc::Receiver;
use std::sync::{mpsc, Arc, Mutex};
use std::thread::{spawn, JoinHandle};

/// Making error struct in case of invalid size of thread-pool
#[derive(Debug)]
struct ThreadPoolSizeError {
    details: String
}
/// Making Display trait for newly created error
impl std::fmt::Display for ThreadPoolSizeError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "ThreadPoolSizeError: {}", self.details)
    }
}
/// Implementing Error trait
impl Error for ThreadPoolSizeError {
    fn description(&self) -> &str {
        &self.details
    }
}

/// Simple thread-pool realization
///
/// Consist of workers vec that represents threads
/// and sender that transfer closures
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take());
        for worker in self.workers.drain(..) {
            println!("Вырубаем {} поток", worker.id);
            worker.thread.join().unwrap()
        }
    }
}

type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    /// Panics if zero
    pub fn build(size: usize) -> Result<ThreadPool, Box<dyn Error>> {
        let size_valid = match size {
            0 => Err(ThreadPoolSizeError {
                details: String::from("invalid thread-pool size: must not be 0")
            }),
            size => Ok(size)
        };
        match size {
            Ok(size)
        }
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        let mut workers = Vec::with_capacity(size);
        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)))
        }
        let str = String::new()
        ThreadPool {
            workers,
            sender: Some(sender),
        }
    }

    /// Sending
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.as_ref().unwrap().send(job).unwrap()
    }
}

/// This struct made for optimising thread-pool management
/// Workers represent threads in thread-pool
pub struct Worker {
    id: usize,
    thread: JoinHandle<()>,
}

impl Worker {
    /// This function creates single thread-pool's thread
    /// and goes on infinity loop of closure awaiting
    ///
    /// # example:
    /// ```
    /// use std::sync::{mpsc, Arc, Mutex};
    /// use thread_pool::Worker;
    ///
    /// let (sender, receiver) = mpsc::channel();
    /// let receiver = Arc::new(Mutex::new(receiver));
    ///
    /// let example_id = 228;
    /// let worker = Worker::build(228, receiver).unwrap();
    /// ```
    ///
    /// mpsc::channel() needs for multiple worker creation
    /// as well as Arc and Mutex
    pub fn build(id: usize, receiver: Arc<Mutex<Receiver<Job>>>) -> Result<Worker, Box<dyn Error>> {
        /// Variable thread will return error in case of panic in thread,
        /// so lets initiate panic with encountered error to make sure
        /// of dealing with it
        let thread = spawn(move || loop {
            let lock_result = receiver.lock();
            match lock_result {
                Ok(mutex) => {
                    let message = mutex.recv();
                    match message {
                        Ok(job) => {
                            println!("Поток {id} выполняет работу");
                            job();
                        }
                        Err(e) => {
                            eprintln!("Поток {id} выключается...");
                            panic!(e);
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Thread error while locking closure: {:#?} at {} thread", e, id);
                    panic!(e);
                }
            }
        });
        match thread {
            Ok(thread) => {
                Ok(Worker { id, thread })
            }
            Err(e) => {
                e
            }
        }
    }
}

#[cfg!(test)]
mod tests {
    use std::thread;
    use std::time::Duration;
    use super::*;

    #[test]
    fn some_thread_work() {
        let pool = ThreadPool::new(4);
        for i in 0..4 {
            pool.execute(||{thread::sleep(Duration::from_secs(3))})
        }
    }
}