use super::schema::posts;
use rocket::{Request, Data};
use rocket::data::{self, FromData};
use rocket::http::Status;
use rocket::Outcome::*;
use serde_json;

#[derive(Queryable)]
#[derive(Serialize, Deserialize)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub pinned: bool,
}


// This enables using PostData from incoming request data
pub struct PostData {
    pub title: String,
    pub body: String,
    pub pinned: bool,
}

impl FromData for PostData {
    type Error = String;
    #[allow(unused_variables)]
    fn from_data(req: &Request, data: Data) -> data::Outcome<Self,String>{
       let reader = data.open();
       match serve_json::from_reader(reader).map(|val| val) {
            Ok(value) => Success(value),
            Err(e) => Failure((Status::BadRequest, e.to_string())),
       }
    }
}

