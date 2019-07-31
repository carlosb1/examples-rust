#![feature(proc_macro_hygiene)]
#![feature(decl_macro)]
#[macro_use] extern crate rocket;

use rocket::local::Client;

#[get("/")]
fn hello() -> &'static str {
    "Hello world"
}

fn main() {
    println!("Hello, world!");
    rocket::ignite().mount("/", routes![hello]).launch();
}



#[test]
fn test1() {
    let rocket = rocket::ignite().mount("/", routes![hello]);
    let client = Client::new(rocket).unwrap();
    let mut response = client.get("/").dispatch();
    assert_eq!(response.body_string(), Some("Hello world".into()));

}
