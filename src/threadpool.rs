// author: https://zhuanlan.zhihu.com/p/658981710
use std::{
    sync::{
        mpsc::{channel, Sender},
        Arc, Mutex,
    },
    thread,
    thread::JoinHandle,
};

type Workfn = Box<dyn FnOnce() -> () + Send + 'static>;
enum Msg {
    Work(Workfn),
    Down,
}
use Msg::*;

// 主构造函数ThreadPool
pub struct ThreadPool {
    size: usize,
    sender: Sender<Msg>,
    threads: Option<Vec<JoinHandle<()>>>,
}
impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        let mut threads = Vec::with_capacity(size);
        let (sender, receiver) = channel();
        let receiver = Arc::new(Mutex::new(receiver));
        for _ in 0..size {
            let p_rec = Arc::clone(&receiver);
            threads.push(thread::spawn(move || loop {
                let f: Msg = p_rec.lock().unwrap().recv().unwrap();
                match f {
                    Work(f) => {
                        f();
                        // println!("{} works", i)
                    }
                    Down => {
                        // println!("{} down", i);
                        break;
                    }
                };
            }));
        }
        ThreadPool {
            size,
            sender,
            threads: Some(threads),
        }
    }
    pub fn exec(&self, f: Workfn) {
        self.sender.send(Work(Box::new(f))).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        for _ in 0..self.size {
            self.sender.send(Down).unwrap();
        }
        //等待所有线程执行完毕
        for thread in self.threads.take().unwrap() {
            thread.join().unwrap();
        }
    }
}
