use diesel;
use diesel::prelude::*;
use schema::tasks;
use diesel::pg::PgConnection;
use models::task::{Task, TaskJson};

#[derive(Insertable)]
#[table("tasks")]
struct NewTask<'a> {
     title: &'a str,
     body: &'a str,
     done: &'a bool,
}

pub fn create (
    conn: &PgConnection,
    title: &str,
    body: &str,
    done: &bool,
    ) -> ArticleJson  {
    let new_task = &NewTask {
        title,
        body,
        done,
    };

    diesel::insert_into(tasks::table).values(new_task).get_result::<Task>(conn).expect("Error creating task");
    // TODO return json
}

pub struct FindTasks {
    title: Option<String>,
    limit: Option<i64>, 
    offset: Option<i64>,
}

pub fn find(conn:: &PgConnection, params: FindTasks) -> Vec<TaskJson> {
    // let mut query = tasks::table.select().into_boxed
    // let result = query.limit();
}

