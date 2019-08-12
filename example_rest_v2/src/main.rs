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
use schema::posts;



#[derive(Debug)]
#[derive(Queryable)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub publihed: bool,
}

#[derive(Insertable)]
#[table_name="posts"]
pub struct NewPost <'a>{
    pub title: &'a str,
    pub body: &'a str,

}

// DB classes 
pub struct DBPost {}

impl DBPost {
    // DATABASE classes
        pub fn establish_connection(self) -> PgConnection {
            dotenv().ok();
            let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
            PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
        }

        pub fn create(self, post: NewPost) -> Post {
            let conn = self.establish_connection();
            diesel::insert_into(posts::table).values(&post).get_result(&conn).expect("Error saving!")
        }
        pub fn read(self) -> Vec<Post> {
            let conn = self.establish_connection();
            posts::table.load::<Post>(&conn).unwrap()
      }
}

impl<'a, 'r> FromRequest<'a, 'r> for DBPost {
    type Error = ();
    fn from_request(_request: &'a Request<'r>) -> Outcome<Self, Self::Error> {
        Success(DBPost{})
    }
}


// Entities classes
pub trait UseCase {
    fn run(&self) -> &'static str;
}

use std::rc::Rc;

struct HelloWorldCase {
    db: Rc<Option<DBPost>>
}

impl HelloWorldCase {
    pub fn new() -> HelloWorldCase {
        HelloWorldCase{db: Rc::new(None)}
    }

}

impl UseCase for HelloWorldCase {
    fn run(&self) -> &'static str {
        //unwrap shared reference
        let values = Rc::try_unwrap(Rc::clone(&self.db));
        let result = match values.unwrap_or(None)  {
            Some(x) => x.read(),
            None => Vec::new(),
        };
        println!("{:?}",result);
        "Hello world"
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for HelloWorldCase {
    type Error = ();
    fn from_request(_request: &'a Request<'r>) -> Outcome<Self, Self::Error> {
        Success(HelloWorldCase::new())
    }
}

#[get("/")]
fn get(db: DBPost) -> &'static str {
    HelloWorldCase{db: Rc::new(Some(db))}.run()
}
/*
#[post("/", format="application/json", data="<post>")]
fn post(db: DBPost, post: NewPost) -> &'static str {
    "post"
}
*/


fn main() {
    rocket::ignite().mount("/", routes![get]).launch();
}


#[test]
fn test1() {
    use rocket::local::Client;
//    let use_case = HelloWorldCase{};
    let db = DBPost{};
    let rocket = rocket::ignite().manage(db).mount("/", routes![get]);
    let client = Client::new(rocket).unwrap();
    let mut response = client.get("/").dispatch();
    assert_eq!(response.body_string(), Some("Hello world".into()));
}
