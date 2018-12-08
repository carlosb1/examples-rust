use super::schema::tasks;

use diesel::prelude::*;
use rocket::{Request, Data};
use rocket::data::{self, FromData};
use rocket::http::{Status};
use rocket::Outcome::*;
use serde_json;

#[derive(Queryable, Identifiable, Serialize, Deserialize)]
pub struct Task {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub done: bool,
}

#[derive(Insertable, AsChangeset, Deserialize)]
#[table_name="tasks"]
pub struct NewTask {
    pub title: String,
    pub body: String,
    pub done: bool,
}

impl FromData for NewTask {
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

impl Task {
    pub fn create(task: NewTask, conn: &PgConnection) -> Task  {
        /*
        let new_task = NewTask {
            title: task.title,
            body: task.body,
            done: task.done,
        };
        */
        diesel::insert_into(tasks::table).values(&task).get_result(conn).expect("Error saving tasks")
    }    
    pub fn read(conn: &PgConnection) -> Vec<Task> {
        tasks::table.load::<Task>(conn).unwrap()   }
    pub fn delete(id: i32, conn: &PgConnection) -> bool {
        diesel::delete(tasks::table.find(id)).execute(conn).is_ok()
    }
    pub fn update(task: NewTask, id: i32, conn: &PgConnection) -> bool{
        diesel::update(tasks::table.find(id)).set(&task).execute(conn).is_ok()
    }
    pub fn get(conn: &PgConnection, id: i32) -> Task {
        tasks::table.find(id).first::<Task>(conn).unwrap()
    }

}


