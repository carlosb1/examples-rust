use diesel::prelude::*;
use std::ops::Deref;
use std::env;
use diesel;
use rocket::request::{Outcome, FromRequest, State};
use rocket::Outcome::{Success, Failure};
use rocket::Request;
use rocket::http::Status;


type Pool = diesel::r2d2::Pool<diesel::r2d2::ConnectionManager<PgConnection>>;
pub struct DB(diesel::r2d2::PooledConnection<diesel::r2d2::ConnectionManager<PgConnection>>);

impl Deref for DB {
    type Target = PgConnection;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for DB {
    type Error = ();
    fn from_request(request: &'a Request<'r>) -> Outcome<Self, Self::Error> {
        let pool = request.guard::<State<Pool>>()?;
        match pool.get() {
            Ok(conn) => Success(DB(conn)),
            Err(_) => Failure((Status::ServiceUnavailable, ())),
        }
    }
}

pub fn init_pool() -> Pool {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = diesel::r2d2::ConnectionManager::<PgConnection>::new(database_url);
    let pool = diesel::r2d2::Pool::builder().build(manager).expect("db pool");
    pool
}

/*
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
*/
