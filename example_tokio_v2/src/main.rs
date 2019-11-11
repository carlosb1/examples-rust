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
use std::sync::{Arc, Mutex};



/* generic interface for protocols */
pub trait MessageProtocolParser {
    fn parse(&self, info: &Vec<u8>);
    fn is_message(&self, info: &Vec<u8>) -> bool;
}


#[derive(Clone,Copy)]
pub struct ExampleJSONParser;
impl ExampleJSONParser {
    fn new() -> ExampleJSONParser {
        ExampleJSONParser{}
    }
}

impl MessageProtocolParser for ExampleJSONParser {
    fn is_message(&self, info: &Vec<u8>) -> bool {
        true
    }
    fn parse(&self, info: &Vec<u8>) {
        let vec_to_parse = info.clone();
        let message = String::from_utf8(vec_to_parse).unwrap();
        println!("Json parser for: {:?}", message);
        let msg: Message = match serde_json::from_str(&message)  {
            Err(..) =>   {println!("It was not parsed correctly"); Message::new_empty() },
            Ok(msg) => msg,
        };
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    operation: String,
}

impl Message {
    fn new(operation: String) -> Message {
        Message {operation: operation} 
    }
    fn new_empty() -> Message  {
        Message {operation: "".to_string()}
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
    type Item = Vec<u8>;
    type Error = io::Error;

    fn decode(&mut self, buf: &mut BytesMut) -> io::Result<Option<Vec<u8>>> {
        if buf.len() == 0 {
            return Ok(None);
        }
        let data = buf.clone().to_vec();
        /*
         * add my logic
        for parser in self.parsers.iter() {
            let cloned_data = data.clone();
            if parser.lock().unwrap().is_message(&cloned_data) {
                parser.lock().unwrap().parse(&cloned_data);
            }
        }
        */
        buf.clear();
        Ok(Some(data))
    }
}

impl Encoder for MyBytesCodec {
    type Item = Vec<u8>;
    type Error = io::Error;

    fn encode(&mut self, data: Vec<u8>, buf: &mut BytesMut) -> io::Result<()> {
        buf.extend(data);
        Ok(())
    }
}
    

fn main() {
    let addr = "127.0.0.1:12345".parse().unwrap();
    let listener = TcpListener::bind(&addr).expect("unable to bind TCP listener");
    
    let json_parser = ExampleJSONParser::new();
    

    let server = listener.incoming()
            .map_err(|e| eprintln!("accept failed = {:?}", e))
            .for_each(move |socket| {
                let framed = MyBytesCodec::new().framed(socket);
                let (_writer, reader) = framed.split();

                let handle_conn = reader.for_each(|bytes| {
                    println!("no modified bytes: {:?}", bytes); 
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
