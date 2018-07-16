use dotenv::dotenv;
use std::env;
use diesel::sqlite::SqliteConnection;
use r2d2;
use r2d2_diesel::ConnectionManager;
use rocket::request::{Outcome, FromRequest};
use rocket::Outcome::{Success, Failure};
use rocket::Request;
use rocket::http::Status;

lazy_static! {
    pub static ref DB_POOL:
        r2d2::Pool<ConnectionManager<SqliteConnection>> = {
                dotenv().ok();
                let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
                let config = r2d2::Config::builder().pool_size(32).build();
                let manager = ConnectionManager::<SqliteConnection>:new(database_url);
                r2d2::Pool::new(config, manager).expect("Failed to create pool."));
        };
}
