#![feature(proc_macro, conservative_impl_trait, generators)]
extern crate bytes;
extern crate futures_await as futures;
extern crate tokio_core;
extern crate tokio_io;


mod cmd;
mod ftp;
mod error;
mod codec;

use std::result;
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
use std::path::PathBuf;


use cmd::Command;
type Writer = SplitSink<Framed<TcpStream, FtpCodec>>;

struct Client {
    cwd: PathBuf,
    writer: Writer,
}

impl Client {
    fn new(writer: Writer) -> Client {
        Client {
            cwd: PathBuf::from("/"),
            writer,
        }
    }
    #[async]
    fn send(mut self, answer: Answer) -> Result<Self> {
        self.writer = await!(self.writer.send(answer))?;
        Ok(self)
    }

    #[async]
    fn handle_cmd(mut self, cmd: Command) -> Result<Self> {
        println!("Received command: {:?}", cmd);
        match cmd {
            Command::User(content) => {
                if content.is_empty() {
                    self = await!(self.send(Answer::new(ResultCode::InvalidParameterOrArgument,"Invalid username")))?;
                } else {
                    self = await!(self.send(Answer::new(ResultCode::UserloggedIn,&format!("Welcome {}",content))))?;
                }
            },
            Command::Pwd => {
                let msg = format!("{}", self.cwd.to_str().unwrap_or(""));
                if !msg.is_empty() {
                    let message = format!("\"/{}\" ",msg);
                    self = await!(self.send(Answer::new(ResultCode::PATHNAMECreated, &message)))?;
                } else {
                    self = await!(self.send(Answer::new(ResultCode::FileNotFound, "No such file or directory")))?;
                }
            },
            Command::Unknown(s) => self = await!(self.send(Answer::new(ResultCode::UnknownCommand, &format!("\"{}\": Not Implemented",s))))?,
            _=>  self = await!(self.send(Answer::new(ResultCode::CommandNotImplemented, "Not implemented")))?,
        }
        return Ok(self);
    }
    
}

#[async]
fn client(stream: TcpStream) -> Result<()>{
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
