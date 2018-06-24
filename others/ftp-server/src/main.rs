#![feature(proc_macro, conservative_impl_trait, generators)]
extern crate bytes;
extern crate futures_await as futures;
extern crate tokio_core;
extern crate tokio_io;


mod cmd;
mod ftp;
mod error;
mod codec;

use std::io;
use futures::prelude::*;
use tokio_core::reactor::Core;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use tokio_core::reactor::Handle;
use tokio_core::net::TcpListener;

use tokio_core::net::TcpStream;

use futures::{Sink, Stream};
use futures::stream::SplitSink;

use tokio_io::AsyncRead;
use tokio_io::codec::Framed;
use codec::FtpCodec;
use error::Result;
use ftp::{Answer, ResultCode};


use cmd::Command;
type Writer = SplitSink<Framed<TcpStream, FtpCodec>>;

struct Client {
    writer: Writer,
}

impl Client {
    fn new(writer: Writer) -> Client {
        Client {
            writer,
        }
    }
    #[async]
    fn handle_cmd(mut self, cmd: Command) -> Result<Self> {
        return Ok(self);
    }
}

#[async]
fn client(stream: TcpStream) -> Result<()> {
    let (writer, reader) = stream.framed(FtpCodec).split();
    let writer = await!(writer.send(Answer::new(ResultCode::ServiceReadyForNewUser, "Welcome to this FTP Server!")))?;
    let mut client = Client::new(writer);
    #[async]
    for cmd in reader {
        client = await!(client.handle_cmd(cmd))?;
    }
    println!("Client closed");
    return Ok(());;
}

#[async]
fn handle_client(stream: TcpStream) -> result::Result<(), ()> {
    await!(client(stream)).map_err(|error| println!("Error handling client: {}", error))
}

#[async]
fn server(handle: Handle) -> io::Result<()> {
    let port = 1234;
    let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127,0,0,1)), port);
    let listener = TcpListener::bind(&addr,&handle)?;
    println!("Waiting clients on port {}...",port);
    #[async]
    for (stream, addr) in listener.incoming() {
        let address = format!("[address : {}]",addr);
        println!("New client {}", address);
        handle.spawn(handle_client(stream));
        println!("Waiting another client...");
    }
    return Ok(());
}



fn main() {
    let mut core = Core::new().expect("Cannot create tokio Core");
    let handle = core.handle();
    if let Err(error) = core.run(server(handle)) {
        println!("Error running the server: {}", error);
    }
    
}
