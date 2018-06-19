#![feature(proc_macro, conservative_impl_trait, generators)]
extern crate bytes;
extern crate futures_await as futures;
extern crate tokio_core;
extern crate tokio_io;


mod cmd;
mod ftp;
mod error;

use std::io;
use futures::prelude::async;

#[async]
fn server() -> io::Result<()> {
    Ok(())
}


use tokio_core::reactor::Core;
fn main() {
    let mut core = Core::new().expect("Cannot create tokio Core");
    if let Err(error) = core.run(server()) {
        println!("Error running the server: {}", error);
    }
    println!("Hello, world!");
}
