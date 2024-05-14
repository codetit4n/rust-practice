use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

pub mod models;
pub mod schema;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let datbase_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set!");
    PgConnection::establish(&datbase_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", datbase_url))
}
