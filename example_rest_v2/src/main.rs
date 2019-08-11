#![feature(proc_macro_hygiene)]
#![feature(decl_macro)]
#[macro_use] extern crate rocket;
#[macro_use] extern crate diesel;
extern crate dotenv;


use rocket::request::{Outcome, FromRequest};
use rocket::Outcome::{Success};
use rocket::Request;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;

pub mod schema; 


// DATABASE classes
pub fn establish_connection() -> PgConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

#[derive(Queryable)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub publihed: bool,
}




// Entities classes
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
