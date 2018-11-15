use std::thread;
use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;

enum Message {
    NewJob(Job),
    Terminate,
}

pub struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        let thread = thread::spawn(move || {
            loop {
                let msg = receiver.lock().unwrap().recv().unwrap();

                match msg {
                    Message::NewJob(job) => {
                        println!("Worker {} got a job; executing.", id);
                        job.call_box()
                    },
                    Message::Terminate => {
                        println!("Worker {} was told to terminate.", id);
                        break
                    },
                }
            }
        });

        Worker {
            id:id,
            thread:Some(thread),
        }
    }
}

pub trait FnBox {
    fn call_box(self: Box<Self>);
}

impl<F:FnOnce()> FnBox for F {
    fn call_box(self: Box<Self>) {
        (*self)();
    }
}

type Job = Box<FnBox + Send + 'static>;

pub struct ThreadPool{
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        println!("Sending terminate message to all workers.");
        for _ in &mut self.workers{
            self.sender.send(Message::Terminate).unwrap();
        }
        println!("Shutting down all workers.");
        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);
            // waitting for execute...
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

impl ThreadPool {

    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);
        let mut workers = Vec::with_capacity(size);

        let (sender, recevier) = mpsc::channel();
        let recevier = Arc::new(Mutex::new(recevier));

        for id in 0 .. size {
            workers.push(Worker::new(id, recevier.clone()));
        }

        ThreadPool{
            workers,
            sender,
        }
    }

    pub fn execute<F>(&self, f:F) 
        where F: FnBox + Send + 'static
    {
        let msg = Message::NewJob(Box::new(f));
        self.sender.send(msg).unwrap();
    }
}