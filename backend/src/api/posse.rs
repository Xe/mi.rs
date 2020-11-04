use super::Result;
use crate::{
    models, paseto, schema,
    web::{DiscordWebhook, Error as WebError, Mastodon, Result as WebResult, Twitter},
    MainDatabase,
};
use diesel::prelude::*;
use rocket::State;
use serde::Deserialize;
use std::fmt::Write;

#[derive(Deserialize)]
pub struct Jsonfeed {
    pub version: String,
    pub home_page_url: String,
    pub items: Vec<Item>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct Item {
    pub url: String,
    pub title: String,
    pub tags: Option<Vec<String>>,
}

impl Item {
    fn render(self) -> String {
        let mut result = String::new();

        write!(result, "{}\n\n{}", self.title, self.url).unwrap();

        if let Some(tags) = self.tags {
            write!(result, "\n\n").unwrap();

            for tag in tags.iter() {
                write!(result, "#{} ", tag).unwrap();
            }
        }

        result
    }
}

impl Into<models::Blogpost> for Item {
    fn into(self) -> models::Blogpost {
        models::Blogpost {
            url: self.url,
            title: self.title,
        }
    }
}

pub fn read_jsonfeed(url: String) -> WebResult<Jsonfeed> {
    let resp = ureq::get(&url)
        .set("User-Agent", crate::APPLICATION_NAME)
        .call();

    if resp.ok() {
        Ok(resp.into_json_deserialize()?)
    } else {
        Err(match resp.synthetic_error() {
            Some(why) => WebError::UReq(why.to_string()),
            None => WebError::HttpStatus(resp.status()),
        })
    }
}

#[instrument(skip(dw, tw, ma), err)]
fn posse(item: Item, dw: &DiscordWebhook, tw: &Twitter, ma: &Mastodon) -> WebResult {
    let message = item.render();

    dw.send(message.clone())?;
    tw.tweet(message.clone())?;
    ma.toot(message.clone())?;

    Ok(())
}

pub static BLOG_FEED_URL: &'static str = "https://christine.website/blog.json";

#[post("/blog/refresh")]
#[instrument(skip(conn, dw, tw, ma), err)]
pub fn refresh_blog(
    tok: paseto::Token,
    conn: MainDatabase,
    dw: State<DiscordWebhook>,
    tw: State<Twitter>,
    ma: State<Mastodon>,
) -> Result {
    use schema::blogposts::dsl::blogposts;
    let feed = read_jsonfeed(BLOG_FEED_URL.to_string())?;

    for item in feed.items.into_iter() {
        match blogposts
            .find(item.url.clone())
            .get_result::<models::Blogpost>(&*conn)
        {
            Ok(_) => continue,
            Err(_) => {
                diesel::insert_into(schema::blogposts::table)
                    .values(&{
                        let post: models::Blogpost = item.clone().into();
                        post
                    })
                    .execute(&*conn)?;
                posse(item, &dw, &tw, &ma)?
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{read_jsonfeed, BLOG_FEED_URL};

    #[test]
    fn valid_jsonfeed() {
        read_jsonfeed(BLOG_FEED_URL.to_string()).unwrap();
    }
}
