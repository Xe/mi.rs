#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate tracing;

use color_eyre::eyre::Result;
use rocket::{fairing::AdHoc, http::Method};
use rocket_contrib::helmet::SpaceHelmet;
use rocket_cors::{AllowedHeaders, AllowedOrigins};
use rocket_oauth2::OAuth2;
use rocket_prometheus::PrometheusMetrics;

use ::mi::{
    api, frontend, gitea, paseto, rocket_trace::*, web::*, wellknown, Gitea, MainDatabase,
    APPLICATION_NAME,
};

fn main() -> Result<()> {
    color_eyre::install()?;
    tracing_subscriber::fmt::init();

    info!("{} starting up", APPLICATION_NAME);

    let allowed_origins = AllowedOrigins::some_exact(&[
        "https://mi.within.website",
        "http://localhost:8000",
        "http://mi.akua",
        "http://shachi.akua.xeserv.us:8000",
    ]);

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
        .attach(SpaceHelmet::default())
        .attach(static_files())
        .attach(frontend::fairing())
        .attach(MainDatabase::fairing())
        .attach(RequestId {})
        .attach(paseto::ed25519_keypair())
        .attach(DiscordWebhook::fairing())
        .attach(Mastodon::fairing())
        .attach(PluralKit::fairing())
        .attach(SwitchCounter::fairing())
        .attach(Twitter::fairing())
        .attach(OAuth2::<Gitea>::fairing("gitea"))
        .attach(Lemmy::fairing())
        .attach(AdHoc::on_launch("systemd readiness", |_| {
            if let Ok(ref mut n) = sdnotify::SdNotify::from_env() {
                let _ = n
                    .notify_ready()
                    .map_err(|why| error!("can't signal readiness to systemd: {}", why))
                    .unwrap();
            }
        }))
        .mount("/metrics", prometheus)
        .mount(
            "/",
            routes![wellknown::botinfo, wellknown::robots, wellknown::security],
        )
        .mount("/login/gitea", routes![gitea::callback, gitea::login])
        .mount(
            "/api",
            routes![
                api::indieauth::auth,
                api::package_tracking::orangeconnex::list,
                api::package_tracking::orangeconnex::recieved,
                api::package_tracking::orangeconnex::status,
                api::package_tracking::orangeconnex::track,
                api::posse::notify,
                api::posse::refresh_blog,
                api::switch::current_front,
                api::switch::current_front_text,
                api::switch::get,
                api::switch::list,
                api::switch::switch,
                api::token::info,
                api::token::mint,
                api::webmention::accept,
                api::webmention::get,
                api::webmention::lookup_target,
                api::webmention::list,
                api::get_members,
                api::tweet,
                api::toot,
            ],
        )
        .launch();

    Ok(())
}
