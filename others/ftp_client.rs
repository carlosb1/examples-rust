use std::net::TcpListener;
use std::io::Write;

fn main () {
    let listener = TcpListener::bind("0.0.0.0:1234").expect("Couldn t bind this address...");
    println!("Waiting for clients to connect..."); 
    for stream in listener.incoming() {
        match stream {
            Ok(mut s) => {
                println!("New client");
                if let Err(_) = s.write(b"hello") {
                    println!("Failed to send hello... :'(");
    
                }
             
            }
            _ => {
             println!("A client tried to connect...")
            }
        }
    }

}

