#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate tracing;

use color_eyre::eyre::Result;
use rocket::http::Method;
use rocket_contrib::helmet::SpaceHelmet;
use rocket_cors::{AllowedHeaders, AllowedOrigins};
use rocket_prometheus::PrometheusMetrics;

use ::mi::{api, paseto, web::*, MainDatabase, APPLICATION_NAME};

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

    let allowed_origins =
        AllowedOrigins::some_exact(&["https://mi.within.website", "http://localhost:8000"]);

    // You can also deserialize this
    let cors = rocket_cors::CorsOptions {
        allowed_origins,
        allowed_methods: vec![Method::Get, Method::Post]
            .into_iter()
            .map(From::from)
            .collect(),
        allowed_headers: AllowedHeaders::some(&["Authorization", "Accept"]),
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors()?;

    let prometheus = PrometheusMetrics::with_registry(prometheus::default_registry().clone());
    rocket::ignite()
        .attach(prometheus.clone())
        .attach(cors)
        .attach(MainDatabase::fairing())
        .attach(SpaceHelmet::default())
        .attach(paseto::ed25519_keypair())
        .attach(DiscordWebhook::fairing())
        .attach(Mastodon::fairing())
        .attach(PluralKit::fairing())
        .attach(SwitchCounter::fairing())
        .attach(Twitter::fairing())
        .mount("/metrics", prometheus)
        .mount("/", routes![botinfo])
        .mount(
            "/api",
            routes![
                api::posse::notify,
                api::posse::refresh_blog,
                api::switch::current_front,
                api::switch::current_front_text,
                api::switch::get,
                api::switch::list,
                api::switch::switch,
                api::webmention::accept,
                api::webmention::get,
                api::webmention::list,
                api::get_members,
                api::token_info,
                api::tweet,
                api::toot,
            ],
        )
        .launch();

    Ok(())
}
