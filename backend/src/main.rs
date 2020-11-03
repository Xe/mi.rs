#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate tracing;

use color_eyre::eyre::Result;
use rocket_contrib::helmet::SpaceHelmet;
use rocket_prometheus::PrometheusMetrics;

use ::mi::{api, web, MainDatabase, APPLICATION_NAME};

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
