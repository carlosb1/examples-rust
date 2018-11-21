#![feature(plugin, custom_derive)]
#[macro_use]extern crate rocket;
#[macro_use]extern crate serde_json;
#[macro_use]extern crate serde_derive;
#[macro_use]extern crate rocket_contrib;

mod routes;


use std::sync::Mutex;
use std::collections::HashMap;



fn main ()  {
    rocket::ignite().mount("/", routes![routes::index, routes::get, 
                           routes::new, routes::update])
        .register(catchers![routes::not_found]).manage(Mutex::new(HashMap::<routes::ID, String>::new())).launch();
}
