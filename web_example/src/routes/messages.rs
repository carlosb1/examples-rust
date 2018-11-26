use std::sync::Mutex;
use std::collections::HashMap;

use rocket::State;
use rocket_contrib::json::{Json, Value};


pub type ID = usize;

type MessageMap = Mutex<HashMap<ID, String>>;

#[derive(Serialize, Deserialize)]
struct Message {
    id: Option<ID>,
    contents: String
}

#[get("/<id>", format="application/json")]
fn get(id: ID, map: State<MessageMap>) -> Option<Json<Message>> {
    let hashmap = map.lock().unwrap();
    return(hashmap.get(&id).map(|contents| {
        Json(Message {
                id: Some(id),
                contents: contents.clone()
            })
    }));
}


#[get("/")]
fn index() -> &'static str {
    return("Hello, world!");
}

#[post("/<id>", format="application/json", data="<message>")]
fn new(id: ID, message: Json<Message>, map: State<MessageMap>) -> Json<Value>{
    let mut hashmap = map.lock().expect("map lock.");
    if hashmap.contains_key(&id) {
        return(Json(json!({
            "status": "error",
            "reason": "ID exists. Try put."})));
    } else  {
        hashmap.insert(id, message.0.contents);
        return(Json(json!({"status": "ok"})));
    }
}

#[put("/<id>", format = "application/json", data="<message>")]
fn update(id: ID, message: Json<Message>, map: State<MessageMap>) -> Option<Json<Value>> {
    let mut hashmap = map.lock().unwrap();
    if hashmap.contains_key(&id) {
        hashmap.insert(id, message.0.contents);
        return(Some(Json(json!({"status" : "ok"}))));
    } else {
        return(None);
    }
}

#[error(404)]
fn not_found() -> Json {
    Json(json!({
        "status": "error",
        "reason": "Resource was not found"
    }))
}


