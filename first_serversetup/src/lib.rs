use std::thread;
use std::sync::{Arc, mpsc, Mutex};

/*Here is the plan:
 The ThreadPool will create a channel and hold on to the sending side of the
 channel.
 Each Worker will hold on to the receiving side of the channel.
 We’ll create a new Job struct that will hold the closures we want to send down
 the channel.
 The execute method will send the job it wants to execute down the sending side
 of the channel.
 In its thread, the Worker will loop over its receiving side of the channel and
 execute the closures of any jobs it receives.
*/

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    // --snip--
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

        // Rust's channel implementation is a multiple producer, single consumer implementation.
        // This means we can’t just clone the consuming end of the channel to fix this
        // code. Even if we could, that is not the technique we would want to use;
        // instead, we want to distribute the jobs across threads by sharing the
        // single receiver among all the workers.
        // Additionally, taking a job off the channel queue involves mutating the
        // receiver, so the threads need a safe way to share and modify receiver;
        // otherwise, we might get race conditions (as covered in Chapter 16).
        // Recall the thread-safe smart pointers discussed in Chapter 16: to share
        // ownership across multiple threads and allow the threads to mutate the value,
        // we need to use Arc<Mutex<T>>. The Arc type will let multiple workers own
        // the receiver, and Mutex will ensure that only one worker gets a job from the
        // receiver at a time.
        let receiver = Arc::new(Mutex::new(receiver));

        // with_capacity function performs the same task as Vec::new but
        // with an important difference: it preallocates space in the vector.
        // Because we know we need to store size elements in the vector, doing
        // this allocation up front is slightly more efficient than using
        // Vec::new, which resizes itself as elements are inserted.
        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool { workers, sender }
    }

    pub fn execute<F>(&self, f: F)
        where
            F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);

        self.sender.send(job).unwrap();
    }
}

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker{
        let thread = thread::spawn( move || loop {
            // we first call lock on the receiver to acquire the mutex, and then
            // we call unwrap to panic on any errors. Acquiring a lock might fail
            // if the mutex is in a poisoned state, which can happen if some
            // other thread panicked while holding the lock rather than releasing
            // the lock. In this situation, calling unwrap to have this thread panic
            // is the correct action to take. Feel free to change this unwrap to an expect
            //
            // If we get the lock on the mutex, we call recv to receive a Job from the
            // channel. A final unwrap moves past any errors here as well,
            // which might occur if the thread holding the sending side of the channel
            // has shut down, similar to how the send method returns Err if the receiving
            // side shuts down.
            //
            // The call to recv blocks, so if there is no job yet, the current thread will
            // wait until a job becomes available. The Mutex<T> ensures that only one Worker
            // thread at a time is trying to request a job.
            let job = receiver.lock().unwrap().recv().unwrap();
            println!("Worker {} got a job; executing..", id);
            job();
        });
        Worker { id, thread }
    }
}