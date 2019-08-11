#![feature(proc_macro_hygiene)]
#![feature(decl_macro)]
#[macro_use] extern crate rocket;

use rocket::request::{Outcome, FromRequest};
use rocket::Outcome::{Success};
use rocket::Request;

pub trait UseCase {
    fn run(&self) -> &'static str;
}



struct HelloWorldCase {
}


impl UseCase for HelloWorldCase {
    fn run(&self) -> &'static str {
        "Hello world"
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for HelloWorldCase {
    type Error = ();
    fn from_request(request: &'a Request<'r>) -> Outcome<Self, Self::Error> {
        Success(HelloWorldCase{})
    }
}

#[get("/")]
fn hello() -> &'static str {
    HelloWorldCase{}.run()
}

fn main() {
    println!("Hello, world!");
    rocket::ignite().mount("/", routes![hello]).launch();
}


#[test]
fn test1() {
    use rocket::local::Client;
    let use_case = HelloWorldCase{};
    let rocket = rocket::ignite().mount("/", routes![hello]);
    let client = Client::new(rocket).unwrap();
    let mut response = client.get("/").dispatch();
    assert_eq!(response.body_string(), Some("Hello world".into()));
}
