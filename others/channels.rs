use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
use std::thread;

static NTHREADS: i32 = 3;

pub struct Hello;
impl Hello {
    pub fn hello_world (self) {
        println!("hello world");
    }
}
fn main () {
    let hello = Hello{};
    thread::spawn(hello.hello_world());

    let (tx, rx): (Sender<i32>, Receiver<i32>) = mpsc::channel();
    let mut children = Vec::new();
    for id in 0..NTHREADS {
        let thread_tx = tx.clone();

        let child = thread::spawn( move || {
            thread_tx.send(id).unwrap();
            println!("thread {} finished ", id);
        });
        children.push(child);
    }

    let mut ids = Vec::with_capacity(NTHREADS as usize);
    for _ in 0..NTHREADS {
        ids.push(rx.recv());
    }

    for child in children {
        child.join().expect("oop!, the childern thread panicked");
    }
    println!("{:?}", ids);
}
