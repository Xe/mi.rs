#[macro_use]
extern crate tracing;

use color_eyre::eyre::Result;
use diesel::prelude::*;

use mi::{api::posse::*, *};

fn main() -> Result<()> {
    color_eyre::install()?;
    tracing_subscriber::fmt::init();

    info!("{} blogpost importer starting up", mi::APPLICATION_NAME);

    let conn = establish_connection();

    let feed = read_jsonfeed(BLOG_FEED_URL.to_string())?;
    let posts: Vec<models::Blogpost> = feed.items.into_iter().map(Into::into).collect();
    diesel::insert_into(schema::blogposts::table)
        .values(&posts)
        .execute(&conn)?;

    Ok(())
}
