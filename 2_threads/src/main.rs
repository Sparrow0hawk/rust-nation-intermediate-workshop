use std::sync::mpsc::Sender;
use std::sync::Arc;
use std::sync::{mpsc, Mutex};
use std::thread;
use std::time::Duration;

struct ThreadPool {
    handles: Vec<thread::JoinHandle<()>>,
    sender: Sender<Box<dyn FnOnce() + 'static + Send>>,
}

impl ThreadPool {
    fn new(number_of_threads: u8) -> Self {
        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        let mut handles = Vec::with_capacity(number_of_threads.into());

        for n in 0..number_of_threads {
            let receiver_clone = receiver.clone();
            handles.push(thread::spawn(move || {
                let task = receiver_clone.lock().unwrap().recv().unwrap();
            }))
        }

        Self { handles, sender }
    }

    fn execute<F>(&self, task: F)
    where
        F: FnOnce() + 'static + Send,
    {
        self.sender.send(Box::new(task)).unwrap()
    }
}

fn main() {
    let pool = ThreadPool::new(10);

    pool.execute(|| {
        thread::sleep(Duration::from_secs(2));
        println!("SLOW Hello from thread");
    });
    for i in 0..15 {
        pool.execute(move || {
            println!("FAST Hello from thread for task: {}", i);
        });
    }

    // First we're making sure enough time is given to threads to execute the tasks
    // Then, replace this line with the `stop` method.
    thread::sleep(Duration::from_secs(3));
    // pool.stop();
}
