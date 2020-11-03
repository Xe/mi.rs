#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate tracing;

use color_eyre::eyre::Result;
use rocket_contrib::helmet::SpaceHelmet;
use rocket_prometheus::PrometheusMetrics;

use ::mi::{api, web, MainDatabase, APPLICATION_NAME};

#[get("/.within/botinfo")]
fn botinfo() -> &'static str {
    r#"Hello, if you are reading this, you have found this URL in your
access logs. If this program is doing something you don't want it to do,
please contact me at me@christine.website.

This service is intended to act as a POSSE[1] syndication server for
various services to various other services.

Every effort is being taken to ensure that the data going through this
server is my own.

I'm sorry if this causes you any inconvenience.

[1]: https://indieweb.org/POSSE

Be well, Creator."#
}

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
                botinfo,
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
