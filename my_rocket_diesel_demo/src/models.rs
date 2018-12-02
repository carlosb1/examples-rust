use schema::tasks;

#[derive(Queryable, Serializable, Deserializable)]
pub struct Task {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub done: bool,
}

#[derive(Insertable, Identifiable, AsChangeset)]
#[table_name="tasks"]
pub struct NewTask<'a> {
    pub title: &'a str,
    pub body: &'a str,
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
    pub fn create(task: Task, conn: &PgConnection) -> Task
        let new_task = NewTask {
            title: task.title,
            body: task.body,
        };
        diesel::insert_into(tasks::table).values(&new_task).get_result(conn).expect("Error saving tasks")
    }    
    pub fn read(conn: &PgConnection) -> Vec<Task> {
        tasks.order(tasks::id).load::<Task>(&conn).expect("error loading tasks")    }
    pub fn delete(id: i32, conn: &PgConnection) -> bool {
        diesel::delete(tasks::table::find(id)).execute(conn).is_ok()
    }
    pub fn update(task: NewTask, id: i32, conn: &PgConnection){
        diesel::update(tasks::table.find(id)).set(&task).execute(conn).is_ok()
    }

}


