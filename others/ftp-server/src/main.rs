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
use std::env;
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
use std::path::StripPrefixError;


use cmd::Command;
type Writer = SplitSink<Framed<TcpStream, FtpCodec>>;

struct Client {
    cwd: PathBuf,
    server_root: PathBuf,
    writer: Writer,
}

impl Client {
    fn new(writer: Writer, server_root: PathBuf) -> Client {
        Client {
            cwd: PathBuf::from("/"),
            server_root,
            writer,
        }
    }
    #[async]
    fn send(mut self, answer: Answer) -> Result<Self> {
        self.writer = await!(self.writer.send(answer))?;
        Ok(self)
    }
    
    fn complete_path(self, path: PathBuf) -> (Self, result::Result<PathBuf, io::Error>) {
        let directory = self.server_root.join(if path.has_root() {
                path.iter().skip(1).collect()
            } else {
                path
        });
        let dir = directory.canonicalize();
        if let Ok(ref dir) = dir {
            if !dir.starts_with(&self.server_root) {
                return (self, Err(io::ErrorKind::PermissionDenied.into()));
            }
        }
        (self,dir)
    }
    fn strip_prefix(self, dir:PathBuf) -> (Self, result::Result<PathBuf, StripPrefixError>) {
        let res = dir.strip_prefix(&self.server_root).map(|p| p.to_path_buf());
        (self,res)
    }

    #[async]
    fn cwd(mut self, directory: PathBuf) -> Result<Self>{
        let path = self.cwd.join(&directory);
        let (new_self, res) = self.complete_path(path);
        self = new_self;
        if let Ok(dir) = res {
            let (new_self, res) = self.strip_prefix(dir);
            self = new_self;
            if let Ok(prefix) = res {
                self.cwd = prefix.to_path_buf();
                self = await!(self.send(Answer::new(ResultCode::Ok, &format!("Directory changed to \"{}\"", directory.display()))))?;
                return Ok(self)
            }
        }
        self = await!(self.send(Answer::new(ResultCode::FileNotFound, "No such file or directory")))?;
        return Ok(self);
    }

    #[async]
    fn handle_cmd(mut self, cmd: Command) -> Result<Self> {
        println!("Received command: {:?}", cmd);
        match cmd {
            Command::Cwd(directory) => {
                self= await!(self.cwd(directory))?;
            },
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
fn client(stream: TcpStream, server_root: PathBuf) -> Result<()>{
    let (writer, reader) = stream.framed(FtpCodec).split();
    let writer = await!(writer.send(Answer::new(ResultCode::ServiceReadyForNewUser, "Welcome to this FTP Server!")))?;
    let mut client = Client::new(writer, server_root);
    #[async]
    for cmd in reader {
        client = await!(client.handle_cmd(cmd))?;
    }
    println!("Client closed");
    return Ok(());;
}

#[async]
fn handle_client(stream: TcpStream, server_root: PathBuf) -> result::Result<(), ()> {
    await!(client(stream, server_root)).map_err(|error| println!("Error handling client: {}", error))
}

#[async]
fn server(handle: Handle, server_root: PathBuf) -> io::Result<()> {
    let port = 1234;
    let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127,0,0,1)), port);
    let listener = TcpListener::bind(&addr,&handle)?;
    println!("Waiting clients on port {}...",port);
    #[async]
    for (stream, addr) in listener.incoming() {
        let address = format!("[address : {}]",addr);
        println!("New client {}", address);
        handle.spawn(handle_client(stream, server_root.clone()));
        println!("Waiting another client...");
    }
    return Ok(());
}



fn main() {
    let mut core = Core::new().expect("Cannot create tokio Core");
    let handle = core.handle();
    match env::current_dir() {
        Ok(server_root) => {
            if let Err(error) = core.run(server(handle, server_root)) {
                println!("Error running the server: {}", error);
            }
        }, 
        Err(e) => println!("Could not start server: {:?}",e),

    }

    
}
