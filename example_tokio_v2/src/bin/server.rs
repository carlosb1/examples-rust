
extern crate example_tokio_v2 as example;
extern crate tokio;
extern crate tokio_codec;

use tokio::prelude::*;
use std::io;
use std::sync::{Mutex, Arc};
use std::collections::LinkedList;
use tokio::net::{TcpStream, TcpListener};
use example::{Message, MyBytesCodec, Operation};
use tokio::codec::{Decoder};


fn respond(req: Message, shared_list: Arc<Mutex<LinkedList<Message>>>) -> Box< dyn Future<Item=Message, Error= io::Error>  + Send>{
    for elem in shared_list.lock().unwrap().iter() {
        if elem.operation == req.operation {
            elem.clone().run();
        }
    }
    Box::new(future::ok(Message::new("".to_string())))
}


fn process (socket: TcpStream, shared_list: Arc<Mutex<LinkedList<Message>>>) {
    let framed = MyBytesCodec::new().framed(socket);
    let (writer, reader) = framed.split();
    let task = writer.send_all(reader.and_then(move | message | {
            let shared_list = shared_list.clone();    
            respond(message, shared_list)
        }))
        .then(|res| {
            if let Err(e) = res {
                println!("Failed to process connection; error = {:?}", e);
            }
            Ok(())
        });

    tokio::spawn(task);
}


fn main() {
    let mut list = LinkedList::new();
    list.push_back(Message::new("register".to_string()));
    let shared_list = Arc::new(Mutex::new(list));

    let addr = "127.0.0.1:12345".parse().unwrap();
    let listener = TcpListener::bind(&addr).expect("unable to bind TCP listener");
    let server = listener.incoming()
            .map_err(|e| eprintln!("accept failed = {:?}", e))
            .for_each(move |socket| {
                let shared_list = shared_list.clone();
                process(socket, shared_list);
                Ok(())
    });
    tokio::run(server);
}

