#[macro_use]
extern crate tracing;

use color_eyre::eyre::Result;
use diesel::prelude::*;

use mi::{web::bridgy::*, *};

fn main() -> Result<()> {
    use schema::webmentions::{dsl::*, table};

    color_eyre::install()?;
    tracing_subscriber::fmt::init();

    info!("{} bridgy migrator starting up", mi::APPLICATION_NAME);

    let conn = establish_connection();

    table
        .load::<models::WebMention>(&conn)?
        .into_iter()
        .for_each(|wm| {
            if !wm.source_url.contains("https://brid-gy.appspot.com") {
                return;
            }

            if wm.source_url.contains("like/twitter") {
                return;
            }

            let resp = ureq::get(&wm.source_url)
                .set("User-Agent", crate::APPLICATION_NAME)
                .set("Mi-Mentioned-Url", &wm.target_url)
                .call()
                .map_err(|why| {
                    error!("can't fetch {}: {}", wm.source_url, why);
                    why
                })
                .unwrap();

            let body = resp.into_string().unwrap();
            let result = parse(&body).unwrap().unwrap();
            info!("{:?}", result);

            diesel::update(webmentions.find(wm.id))
                .set(&models::UpdateWebMentionSource {
                    source_url: result.target,
                })
                .execute(&conn)
                .unwrap();
        });

    Ok(())
}
