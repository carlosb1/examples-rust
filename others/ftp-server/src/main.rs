//#![feature(proc_macro, conservative_impl_trait, generators)]
#![feature(proc_macro_non_items, conservative_impl_trait, generators)]
extern crate bytes;
extern crate futures_await as futures;
extern crate tokio_core;
extern crate tokio_io;


mod cmd;
mod ftp;
mod error;

use std::io;
use futures::prelude::*;

/*
#[async]
fn server() -> io::Result<()> {
    Ok(())
}
*/


use tokio_core::reactor::Core;
fn main() {
    let mut core = Core::new().expect("Cannot create tokio Core");
    let server = async_block! {
        Ok(())
    };
    let result: Result<_, io::Error> = core.run(server);
    if let Err(error) = result {
        println!("Error running the server: {}", error);
    }

}
