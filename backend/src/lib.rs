#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate tracing;

use diesel::{prelude::*, SqliteConnection};

pub const APPLICATION_NAME: &str = concat!(
    env!("CARGO_PKG_NAME"),
    "/",
    env!("CARGO_PKG_VERSION"),
    " +https://mi.within.website/.within/botinfo"
);

pub mod api;
pub mod frontend;
pub mod models;
pub mod paseto;
pub mod rocket_trace;
pub mod schema;
pub mod web;

#[database("main_data")]
pub struct MainDatabase(SqliteConnection);

pub fn establish_connection() -> SqliteConnection {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}
