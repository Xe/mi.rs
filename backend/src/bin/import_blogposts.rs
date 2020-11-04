#[macro_use]
extern crate tracing;

use color_eyre::eyre::Result;
use diesel::{prelude::*, SqliteConnection};
use std::env;

use mi::{api::posse::*, *};

fn main() -> Result<()> {
    let _ = kankyo::init();
    color_eyre::install()?;
    tracing_subscriber::fmt::init();

    info!("{} blogpost importer starting up", mi::APPLICATION_NAME);

    let conn = establish_connection();

    let feed = read_jsonfeed(BLOG_FEED_URL.to_string())?;
    let posts: Vec<models::Blogpost> = feed
        .items
        .into_iter()
        .map(|item| {
            let post: models::Blogpost = item.into();
            post
        })
        .collect();
    diesel::insert_into(schema::blogposts::table)
        .values(&posts)
        .execute(&conn)?;

    Ok(())
}

pub fn establish_connection() -> SqliteConnection {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}
