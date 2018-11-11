#![feature(proc_macro_hygiene, decl_macro)]

extern crate rocket;
extern crate rocket_contrib;
extern crate serde;

#[macro_use]extern crate serde_derive;
#[macro_use]extern crate serde_json;

use rocket::{get, routes};

#[get("/")]
fn index() -> &'static str {
    return("Hello, world!");
} 

fn main () {
    rocket::ignite().mount("/", routes![index]).launch();
}
