

#[derive(Queryable)]
pub struct Task {
    pub id:i32,
    pub title: String,
    pub body: String,
    pub done: bool,
}

impl Task {
    pub fn attach(self) -> TaskJson {
        return(TaskJson {id: self.id, title: self.title
            , body: self.body, done: self.done});
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TaskJson {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub done: bool,
}
