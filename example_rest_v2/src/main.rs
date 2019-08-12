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

use rocket_contrib::json::Json;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;

use rocket::http::ContentType;


pub mod schema;
use schema::posts;

use std::rc::Rc;

#[derive(Serialize)]
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
#[derive(Clone)]
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
      pub fn clear(self)  {
            let conn = self.establish_connection();
            diesel::delete(posts::table).execute(&conn);
      }
}

impl<'a, 'r> FromRequest<'a, 'r> for DBPost {
    type Error = ();
    fn from_request(_request: &'a Request<'r>) -> Outcome<Self, Self::Error> {
        Success(DBPost{})
    }
}


struct HelloWorldCase {
    db: Rc<Option<DBPost>>
}

impl HelloWorldCase {
    pub fn new() -> HelloWorldCase {
        HelloWorldCase{db: Rc::new(None)}
    }
    pub fn run(&self) -> Vec<Post> {
        //unwrap shared reference
        let db = DBPost{};
        let result = db.read();
        result   
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
    pub fn run(&self) -> Post {
        //TODO find best way to do it... added generics?
        let db = DBPost{};
        let result = db.create(self.post.clone());
        result
    }
}




impl<'a, 'r> FromRequest<'a, 'r> for HelloWorldCase {
    type Error = ();
    fn from_request(_request: &'a Request<'r>) -> Outcome<Self, Self::Error> {
        Success(HelloWorldCase::new())
    }
}

#[get("/")]
fn get(db: DBPost) -> Json<Vec<Post>> {
    Json(HelloWorldCase{db: Rc::new(Some(db))}.run())
}
#[post("/", format="application/json", data="<post>")]
fn post(db: DBPost, post: NewPost) -> Json<Post> {
    Json(AddNewPostCase{db: Rc::new(Some(db)), post:post}.run())
}


fn main() {
    let db = DBPost{};
    rocket::ignite().manage(db).mount("/", routes![get, post]).launch();
}


#[test]
fn test1() {
    use rocket::local::Client;
    let db = DBPost{};
    let rocket = rocket::ignite().manage(db).mount("/", routes![get, post]);
    let client = Client::new(rocket).unwrap();
    let mut response = client.get("/").dispatch();
    assert_eq!(response.body_string().is_some(),true);
}


#[test]
fn test2() {
    use rocket::local::Client;
    let db = DBPost{};
    db.clone().clear();
    let rocket = rocket::ignite().manage(db).mount("/", routes![get, post]);
    let client = Client::new(rocket).unwrap();
    let mut response = client.post("/").header(ContentType::JSON).body("{\"title\": \"mytitle1\", \"body\": \"mybody1\"}").dispatch();
    assert_eq!(response.body_string().is_some(), true);
}
