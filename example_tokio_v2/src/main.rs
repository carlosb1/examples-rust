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
    let addr = "127.0.0.1:12345".parse().unwrap();
    let listener = TcpListener::bind(&addr).expect("unable to bind TCP listener");
    let server = listener.incoming()
            .map_err(|e| eprintln!("accept failed = {:?}", e))
            .for_each(move |socket| {
                let framed = MyBytesCodec::new().framed(socket);
                let (_writer, reader) = framed.split();

                /* function to handle connection  */
                let handle_conn = reader.for_each(|message| {
                    println!("my parsed message!!: {:?}", serde_json::to_string(&message)); 
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

