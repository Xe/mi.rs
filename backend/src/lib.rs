#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate tracing;

use diesel::sqlite::SqliteConnection;

pub const APPLICATION_NAME: &str = concat!(
    env!("CARGO_PKG_NAME"),
    "/",
    env!("CARGO_PKG_VERSION"),
    " +https://mi.within.website/.within/botinfo"
);

pub mod api;
pub mod models;
pub mod schema;
pub mod web;

#[database("main_data")]
pub struct MainDatabase(SqliteConnection);
