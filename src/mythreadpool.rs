use std::thread;
use std::sync::mpsc;
use std::sync::{Arc, Mutex};

struct Data {
    a: f64,
    b: f64,
}

enum Task {
    Execute(Box<dyn FnOnce() + Send + 'static>),
    Terminate,
}

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Task>,
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

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool { workers, sender }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let task = Task::Execute(Box::new(f));
        self.sender.send(task).unwrap();
    }

   /* pub fn wait_all(&self) {
        for worker in &self.workers {
            if let Some(handle) = &worker.handle {
                handle.join().unwrap();
            }
        }
    }*/
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
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Task>>>) -> Worker {
        let handle = thread::spawn(move || {
            println!("Worker {} is created.", id);
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
            }
        });

        Worker { id, handle: Some(handle) }
    }
}


