#![feature(plugin)]
#![plugin(rocket_codegen)]
extern crate rocket;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_codegen;
extern crate dotenv;
extern crate serde_json;
#[macro_use]
extern crate lazy_static;
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;
extern crate r2d2;
extern crate r2d2_diesel;

mod schema;
mod db;
mod post;
mod models;
mod error;

use db::{establish_connection};
use models::{Task, NewTask};
use rocket_contrib::Json;
use rocket::response::status::{Created, NoContent};
use rocket::Rocket;

#[get("/tasks", format = "application/json")]
fn tasks_get(db: PgConnection) -> Result<Json<Vec<Post>>, ApiError> {
    Ok(Json(Task::read(&db)))
}

#[get("/tasks/<id>", format = "application/json")]
fn task_get(db: PgConnection, id: i32) -> Result<Json<Post>, ApiError> {
 //   let post = Task::read(&db, id)?;
 //   Ok(Json(post))
}

#[post("/tasks", format = "application/json", data = "<post>")]
fn task_create(db: PgConnection,  task: NewTask) -> Result<Created<String>, ApiError> {
    let post = Task::create(task &db);
    let url = format!("/task/{}", task);
    Ok(Created(url, Some("Done".to_string())))
}

#[patch("/tasks/<id>", format = "application/json", data = "<post>")]
fn task_edit(db: PgConnection, id: i32, task: NewTask) -> Result<Json<bool>, ApiError> {
    let post = Task::update(post,id, &db);
    Ok(Json(post))
}

#[delete("/tasks/<id>")]
fn task_delete(db: PgConnection, id: i32) -> Result<NoContent, ApiError> {
    Task::delete(id, &db)?;
    Ok(NoContent)
}

fn rocket() -> Rocket {
    rocket::ignite().manage(db::connect()).mount("/", routes![post_create, posts_get, post_delete, post_edit, post_get])
}

fn main() {
        rocket().launch();
}
