use std::thread;
use std::sync::mpsc;
use std::sync::{Arc, Mutex, Condvar};

enum Task {
    Execute(Box<dyn FnOnce() + Send + 'static>),
    Terminate,
}

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Task>,
    semaphore: Arc<(Mutex<usize>, Condvar)>,
}

struct Worker {
    id: usize,
    handle: Option<thread::JoinHandle<()>>,
}

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        let mut workers = Vec::with_capacity(size);
        let semaphore = Arc::new((Mutex::new(0), Condvar::new()));

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver), Arc::clone(&semaphore)));
        }

        ThreadPool { workers, sender, semaphore }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let task = Task::Execute(Box::new(f));

        let (lock, cvar) = &*self.semaphore;
        let mut count = lock.lock().unwrap();

        *count += 1;
        self.sender.send(task).unwrap();

        while *count > self.workers.len() {
            count = cvar.wait(count).unwrap();
        }
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        for _ in &self.workers {
            self.sender.send(Task::Terminate).unwrap();
        }

        for worker in &mut self.workers {
            if let Some(handle) = worker.handle.take() {
                handle.join().unwrap();
            }
        }
    }
}

impl Worker {
    fn new(
        id: usize,
        receiver: Arc<Mutex<mpsc::Receiver<Task>>>,
        semaphore: Arc<(Mutex<usize>, Condvar)>,
    ) -> Worker {
        let handle = thread::spawn(move || {
            println!("Execute with thread {}", id);
            loop {
                let task = receiver.lock().unwrap().recv().unwrap();

                match task {
                    Task::Execute(f) => {
                        f();
                    }
                    Task::Terminate => {
                        break;
                    }
                }

                let (lock, cvar) = &*semaphore;
                let mut count = lock.lock().unwrap();
                *count -= 1;
                cvar.notify_one();
            }
        });

        Worker { id, handle: Some(handle) }
    }
}
