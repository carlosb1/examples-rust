#![feature(proc_macro_hygiene, decl_macro)]

extern crate rocket_contrib;
extern crate serde;

#[macro_use]extern crate rocket;
#[macro_use]extern crate serde_derive;
#[macro_use]extern crate serde_json;

use std::sync::Mutex;
use std::collections::HashMap;

use rocket::State;
use rocket_contrib::json::{Json, JsonValue};


type ID = usize;

type MessageMap = Mutex<HashMap<ID, String>>;

#[derive(Serialize, Deserialize)]
struct Message {
    id: Option<ID>,
    contents: String
}

#[get("/<id>", format="json")]
fn get(id: ID, map: State<MessageMap>) -> Option<Json<Message>> {
    let hashmap = map.lock().unwrap();
    hashmap.get(&id).map(|contents| {
        Json(Message {
                id: Some(id),
                contents: contents.clone()
            })
    })
}


#[get("/")]
fn index() -> &'static str {
    return("Hello, world!");
}

#[catch(404)]
fn not_found() -> JsonValue {
    JsonValue(json!({
        "status": "error",
        "reason": "Resource was not found"
    }))
}

fn main () {
    rocket::ignite().mount("/", routes![index, get]).register(catchers![not_found]).manage(Mutex::new(HashMap::<ID, String>::new())).launch();
}
