use super::{Error, Result};
use crate::{models, schema, MainDatabase};
use diesel::prelude::*;
use rocket::{
    request::Form,
    response::{self, Responder},
    Request, Response,
};
use rocket_contrib::json::Json;
use rusty_ulid::generate_ulid_string;
use url::Url;

#[derive(FromForm, Debug)]
pub struct WebMention {
    source: String,
    target: String,
}

impl WebMention {
    fn check(&self) -> Result {
        if self.source == self.target {
            return Err(Error::InvalidWebMention("source == target".into()));
        }

        let u: Url = Url::parse(&self.source)?;
        match u.scheme() {
            "http" | "https" => {}
            _ => return Err(Error::InvalidWebMention("invalid source scheme".into())),
        }

        u.host_str()
            .ok_or(Error::InvalidWebMention("no host found in target".into()))?;

        let u: Url = Url::parse(&self.target)?;
        match u.scheme() {
            "http" | "https" => {}
            _ => return Err(Error::InvalidWebMention("invalid target scheme".into())),
        }

        match u
            .host_str()
            .ok_or(Error::InvalidWebMention("no host found in target".into()))?
        {
            "christine.website" | "cetacean.club" => {}
            _ => return Err(Error::InvalidWebMention("invalid target host".into())),
        }

        Ok(())
    }
}

impl Into<models::WebMention> for WebMention {
    fn into(self) -> models::WebMention {
        models::WebMention {
            id: generate_ulid_string(),
            source_url: self.source,
            target_url: self.target,
        }
    }
}

impl<'a> Responder<'a> for models::WebMention {
    fn respond_to(self, _: &Request) -> response::Result<'a> {
        Response::build()
            .raw_header(
                "Location",
                format!("https://mi.christine.website/api/webmention/{}", self.id),
            )
            .ok()
    }
}

#[post("/webmention/accept", data = "<mention>")]
#[instrument(skip(conn, mention), err)]
pub fn accept(conn: MainDatabase, mention: Form<WebMention>) -> Result<models::WebMention> {
    use schema::webmentions;

    let mention = mention.into_inner();
    mention.check()?;

    info!(
        source = &mention.source[..],
        target = &mention.target[..],
        "webmention received"
    );

    let wm: models::WebMention = mention.into();
    diesel::insert_into(webmentions::table)
        .values(&wm)
        .execute(&*conn)
        .map_err(Error::Database)?;

    Ok(wm)
}

#[get("/webmention/<mention_id>")]
#[instrument(skip(conn), err)]
pub fn get(conn: MainDatabase, mention_id: String) -> Result<Json<models::WebMention>> {
    use schema::webmentions::dsl::webmentions;

    Ok(Json(
        webmentions
            .find(mention_id)
            .get_result(&*conn)
            .map_err(Error::Database)?,
    ))
}
