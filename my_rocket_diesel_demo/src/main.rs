#![feature(plugin, custom_attribute, uniform_paths)]
#![allow(proc_macro_derive_resolution_fallback)]
#![plugin(rocket_codegen)]
extern crate rocket;
#[macro_use]
extern crate serde_json;
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate diesel;
extern crate dotenv;



mod schema;
mod db;
mod models;
mod error;

use db::DB;
use models::{Task, NewTask};
use rocket_contrib::{Json, Value};
use rocket::Rocket;
use error::ApiError;

#[get("/tasks", format = "application/json")]
fn tasks_get(db: DB) -> Result<Json<Vec<Task>>, ApiError> {
    Ok(Json(Task::read(&db)))
}

#[get("/tasks/<id>", format = "application/json")]
fn task_get(db: DB, id: i32) -> Result<Json<Task>, ApiError> {
    let post = Task::get(&db, id);
    Ok(Json(post))
}

#[post("/tasks", format = "application/json", data = "<task>")]
fn task_create(db: DB,  task: NewTask) -> Result<Json<Task>, ApiError> {
    let new_task = Task::create(task, &db);
    Ok(Json(new_task))
}

#[patch("/tasks/<id>", format = "application/json", data = "<task>")]
fn task_edit(db: DB, id: i32, task: NewTask) -> Result<Json<bool>, ApiError> {
    let new_task = Task::update(task,id, &db);
    Ok(Json(new_task))
}

#[delete("/tasks/<id>")]
fn task_delete(db: DB, id: i32) -> Result<Json<Value>, ApiError> {
    let json_result = Json(json!({"success": Task::delete(id, &db)}));
    Ok(json_result)
}

fn rocket() -> Rocket {
    rocket::ignite().manage(db::init_pool()).mount("/", routes![task_create, task_delete, task_edit, tasks_get, task_get])
}

fn main() {
        rocket().launch();
}
