#[macro_use]
extern crate serde_derive;

extern crate bytes;
extern crate tokio;
extern crate tokio_codec;
extern crate tokio_io;


extern crate serde;
extern crate serde_json;

use bytes::BytesMut;
use std::io;
use tokio::codec::{Encoder,Decoder};
use tokio::prelude::*;
use tokio::net::TcpListener;
use std::sync::{Mutex, Arc};
use std::collections::LinkedList;
use std::rc::Rc;


#[derive(Clone,Copy)]
pub struct ExampleJSONParser;
impl ExampleJSONParser {
    fn new() -> ExampleJSONParser {
        ExampleJSONParser{}
    }
}

impl ExampleJSONParser {
    fn parse(&self, info: &Vec<u8>)  -> Message {
        let vec_to_parse = info.clone();
        let message = String::from_utf8(vec_to_parse).unwrap();
        println!("Json parser for: {:?}", message);
        let msg: Message = match serde_json::from_str(&message)  {
            Err(..) =>   {println!("It was not parsed correctly"); Message::new_empty() },
            Ok(msg) => msg,
        };
        return msg
    }
}

#[derive(Clone)]
#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    operation: String,
}

impl Message{
    fn new(operation: String) -> Message {
        Message {operation: operation} 
    }
    fn new_empty() -> Message  {
        Message {operation: "".to_string()}
    }
    
}

// traits for operations 
pub trait Operation {
    fn run(self);
}

impl Operation for Message {
    fn run (self) {
        println!("hello!");
    }
}

pub struct MyBytesCodec {
    json_parser: ExampleJSONParser,
}

impl MyBytesCodec {
    fn new() -> MyBytesCodec {
        MyBytesCodec{json_parser: ExampleJSONParser::new()}
    }
}

impl Decoder for MyBytesCodec {
    type Item = Message;
    type Error = io::Error;

    fn decode(&mut self, buf: &mut BytesMut) -> io::Result<Option<Message>> {
        if buf.len() == 0 {
            return Ok(None);
        }
        let data = buf.clone().to_vec();
        let cloned_data = data.clone();
        let parsed = self.json_parser.parse(&cloned_data); 
        buf.clear();
        Ok(Some(parsed))
    }
}

impl Encoder for MyBytesCodec {
    type Item = Message;
    type Error = io::Error;

    fn encode(&mut self, data: Message, buf: &mut BytesMut) -> io::Result<()> {
        buf.extend(serde_json::to_string(&data)?.into_bytes());
        Ok(())
    }
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
                let framed = MyBytesCodec::new().framed(socket);
                let (writer, reader) = framed.split();
                let list = shared_list.clone();
                /* function to handle connection  */
                let handle_conn = reader.for_each(move |message| {
                    //writer.send(Message::new("".to_string()));
                    println!("my parsed message!!: {:?}", serde_json::to_string(&message)); 
                    for elem in list.lock().unwrap().iter() {
                        if elem.operation == message.operation  {
                            elem.clone().run();
                        }
                    }
                    Ok(())
                })
                .and_then(|()| {
                    println!("Socket received FIN packet and closed connection");
                    Ok(())
                })
                .or_else(|err| {
                    println!("Socked closed with error: {:?}", err);
                    Err(err)

                })
                .then(|result| {
                    println!("Socket closed with result: {:?}", result);
                    Ok(())
                });
               
            tokio::spawn(handle_conn)
    });
    tokio::run(server);
}

