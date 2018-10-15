#![feature(plugin)]
#![plugin(rocket_codegen)]


#[macro_use] extern crate diesel;
extern crate dotenv;

pub mod schema;
pub mod models;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;

extern crate hello_rocket;
use self::hello_rocket::*;
use self::models::*;
use self::diesel::prelude::*;



pub fn establish_connection() -> PgConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;


use rocket_contrib::{Json, Value};

mod hero;
use hero::{Hero};

#[post("/", data="<hero>")]
fn create (hero: Json<Hero>) -> Json<Hero> {
    hero
}

#[get("/")]
fn read() -> Json<Value> {
    Json(json!([
        "hero 1",
        "hero 2"
    ]))
}

#[put("/<id>", data="<hero>")]
fn update (id: i32, hero: Json<Hero>) -> Json<Hero> {
    hero
}

#[delete("/<id>")]
fn delete(id: i32) -> Json<Value>{
    Json(json!({"status": "ok"}))
}


fn main() {
    rocket::ignite().mount("/hero", routes![create, update, delete])
        .mount("/heroes", routes![read]).launch();
    use diesel_demo::schema::posts::dsl::*;
    let connection = establish_connection();
    let results = posts.filter(published.eq(true)).limit(5).load::<Post>(&connection).expect("Error loading posts");

    println!("Displaying {} posts",results.len());
    for post in results {
        println!("{}", post.title);
        println!("---------------\n");
        println!("{}", post.body);
    }
}


