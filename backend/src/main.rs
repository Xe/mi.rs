#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate tracing;

use color_eyre::eyre::Result;
use diesel::sqlite::SqliteConnection;
use rocket_contrib::helmet::SpaceHelmet;
use rocket_prometheus::PrometheusMetrics;

const APPLICATION_NAME: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));

pub mod api;
pub mod models;
pub mod schema;
pub mod web;

#[database("main_data")]
pub struct MainDatabase(SqliteConnection);

fn main() -> Result<()> {
    let _ = kankyo::init();
    color_eyre::install()?;
    tracing_subscriber::fmt::init();

    info!("{} starting up", APPLICATION_NAME);

    let prometheus = PrometheusMetrics::with_registry(prometheus::default_registry().clone());
    rocket::ignite()
        .attach(prometheus.clone())
        .attach(MainDatabase::fairing())
        .attach(SpaceHelmet::default())
        .attach(web::pluralkit::Client::fairing())
        .attach(web::switchcounter::Client::fairing())
        .mount("/metrics", prometheus)
        .mount(
            "/",
            routes![
                api::get_members,
                api::get_switches,
                api::get_switch,
                api::get_current_front,
                api::make_switch
            ],
        )
        .launch();

    Ok(())
}
