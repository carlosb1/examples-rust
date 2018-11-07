extern crate web_example;
extern crate diesel;

use self::web_example::*;
use self::models::*;
use self::diesel::prelude::*;


fn main ()  {
    use web_example::schema::tasks::dsl::*;

    let connection = establish_connection();
    let results = tasks.filter(done.eq(true)).limit(5).load::<Task>(&connection).expect("Error loading tasks");
    println!("Displaying {} tasks", results.len());

}
