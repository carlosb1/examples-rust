#![feature(proc_macro_hygiene)]
#![feature(decl_macro)]
#![feature(rustc_private)]
#[macro_use] extern crate rocket;
#[macro_use] extern crate diesel;
extern crate serde_json;
extern crate serde;
extern crate dotenv;


use rocket::request::{Outcome, FromRequest};
use rocket::Outcome::Success;
use rocket::Request;

use rocket::Data;
use rocket::data::{self, FromDataSimple};
use rocket::http::Status;
use rocket::Outcome::Failure;
use serde::{Deserialize, Serialize};

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;

use rocket::http::ContentType;


pub mod schema;
use schema::posts;

use std::rc::Rc;

#[derive(Debug)]
#[derive(Queryable)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub publihed: bool,
}

#[derive(Serialize, Deserialize)]
#[derive(Insertable)]
#[derive(Clone)]
#[derive(Debug)]
#[table_name="posts"]
pub struct NewPost{
    pub title: String,
    pub body: String,
}

impl FromDataSimple for NewPost {
    type Error = String;
    
    #[allow(unused_variables)]
    fn from_data(req: &Request, data: Data) -> data::Outcome<Self, String> {
        let reader = data.open();
        match serde_json::from_reader(reader).map(|val| val) {
            Ok(value) => Success(value),
            Err(e) => Failure((Status::BadRequest, e.to_string())),
        }
    }
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
        /*
        let values = Rc::try_unwrap(Rc::clone(&self.db));
        let result = match values.unwrap_or(None)  {
            Some(unwrapped_db) => unwrapped_db.read(),
            None => Vec::new(),
        };
        */
        let db = DBPost{};
        let result = db.read();
        println!("{:?}",result);
        "Hello world"
    }
}


struct AddNewPostCase {
    db: Rc<Option<DBPost>>,
    post: NewPost
}
impl AddNewPostCase  {
    pub fn new(post: NewPost) -> AddNewPostCase {
        AddNewPostCase{db: Rc::new(None), post: post}
    }
}


impl UseCase for AddNewPostCase {
    fn run(&self) -> &'static str {
        //unwrap shared reference
        /*
        let unwrap_db = Rc::try_unwrap(Rc::clone(&self.db));
        let created_post = match unwrap_db {
            Ok(v) => Some(v.unwrap().create(self.post.clone())),
            Err(_) => {println!("It was not possible ost the result"); None},
        };
        */
        //TODO find best way to do it... added generics?
        let db = DBPost{};
        let result = db.create(self.post.clone());
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
#[post("/", format="application/json", data="<post>")]
fn post(db: DBPost, post: NewPost) -> &'static str {
    println!("{:?}", post);
    AddNewPostCase{db: Rc::new(Some(db)), post:post}.run()
}


fn main() {
    let db = DBPost{};
    rocket::ignite().manage(db).mount("/", routes![get, post]).launch();
}


#[test]
fn test1() {
    use rocket::local::Client;
//    let use_case = HelloWorldCase{};
    let db = DBPost{};
    let rocket = rocket::ignite().manage(db).mount("/", routes![get, post]);
    let client = Client::new(rocket).unwrap();
    let mut response = client.get("/").dispatch();
    assert_eq!(response.body_string(), Some("Hello world".into()));
}


#[test]
fn test2() {
    use rocket::local::Client;
//    let use_case = HelloWorldCase{};
    let db = DBPost{};
    let rocket = rocket::ignite().manage(db).mount("/", routes![get, post]);
    let client = Client::new(rocket).unwrap();
    let mut response = client.post("/").header(ContentType::JSON).body("{\"title\": \"mytitle1\", \"body\": \"mybody1\"}").dispatch();
    assert_eq!(response.body_string(), Some("Hello world".into()));
}
