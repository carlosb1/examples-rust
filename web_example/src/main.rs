#![feature(plugin, custom_derive)]
#![plugin(rocket_codegen)]

#[macro_use]extern crate serde_derive;
extern crate serde_json;

extern crate rocket;
#[macro_use]extern crate rocket_contrib;

extern crate validator;
#[macro_use]
extern crate validator_derive;

#[macro_use]
extern crate diesel;

extern crate dotenv;

mod models;
mod routes;


use std::sync::Mutex;
use std::collections::HashMap;

use rocket_contrib::{Json, Value};

fn main ()  {
    rocket::ignite().mount("/", routes![routes::messages::index, routes::messages::get, 
                           routes::messages::new, routes::messages::update])
        .catch(errors![routes::messages::not_found]).manage(Mutex::new(HashMap::<routes::messages::ID, String>::new())).launch();
}
