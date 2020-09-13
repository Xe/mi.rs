#[macro_use]
extern crate diesel;

use color_eyre::eyre::Result;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use hyper::{header::CONTENT_TYPE, Body, Response};
use prometheus::{Encoder, TextEncoder};
use warp::Filter;

const APPLICATION_NAME: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));
include!(concat!(env!("OUT_DIR"), "/templates.rs"));

pub mod schema;
pub mod models;

#[tokio::main]
async fn main() -> Result<()> {
    let _ = kankyo::init();
    pretty_env_logger::init();
    color_eyre::install()?;

    log::info!("starting up {} commit {}", APPLICATION_NAME, env!("GITHUB_SHA"));

    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url));

    let healthcheck = warp::get().and(warp::path(".within").and(warp::path("health")).map(|| "OK"));
    let metrics_endpoint = warp::path("metrics").and(warp::path::end()).map(move || {
        let encoder = TextEncoder::new();
        let metric_families = prometheus::gather();
        let mut buffer = vec![];
        encoder.encode(&metric_families, &mut buffer).unwrap();
        Response::builder()
            .status(200)
            .header(CONTENT_TYPE, encoder.format_type())
            .body(Body::from(buffer))
            .unwrap()
    });

    let site = healthcheck
        .or(metrics_endpoint)
        .map(|reply| warp::reply::with_header(reply, "X-Clacks-Overhead", "GNU Ashlynn"))
        .with(warp::log(APPLICATION_NAME));

    warp::serve(site).run(([0, 0, 0, 0], 5000)).await;

    Ok(())
}
