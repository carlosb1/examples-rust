#[macro_use]
extern crate serde_derive;

extern crate bytes;
extern crate tokio;
extern crate tokio_proto as proto;
extern crate tokio_codec;
extern crate tokio_io;


extern crate serde;
extern crate serde_json;

use bytes::BytesMut;
use std::io;
use tokio::codec::{Encoder,Decoder};



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
    pub operation: String,
}

impl Message{
    pub fn new(operation: String) -> Message {
        Message {operation: operation} 
    }
    pub fn new_empty() -> Message  {
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
    pub fn new() -> MyBytesCodec {
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

