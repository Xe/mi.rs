use super::{Error, Result};
use crate::{models, paseto, schema, web::discord_webhook::Client as DiscordWebhook, MainDatabase};
use diesel::prelude::*;
use rocket::{
    request::Form,
    response::{self, Responder},
    Request, Response, State,
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
#[instrument(skip(conn, mention, dw), err)]
pub fn accept(
    conn: MainDatabase,
    mention: Form<WebMention>,
    dw: State<DiscordWebhook>,
) -> Result<models::WebMention> {
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

    dw.send(format!(
        "<{}> mentioned <{}> (<https://mi.within.website/api/webmention/{}>)",
        wm.source_url, wm.target_url, wm.id
    ))
    .map_err(|why| {
        error!("webhook send failed: {}", why);
        Error::Web(why)
    })?;

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

#[get("/webmention?<count>&<page>")]
#[instrument(skip(conn), err)]
pub fn list(
    conn: MainDatabase,
    count: Option<i64>,
    page: Option<i64>,
    tok: paseto::Token,
) -> Result<Json<Vec<models::WebMention>>> {
    use schema::webmentions;

    let count = count.unwrap_or(30);
    let page = page.unwrap_or(0);

    let count = if count < 100 { count } else { 100 };

    Ok(Json(
        webmentions::table
            .limit(count)
            .offset(count * (page - 1))
            .load::<models::WebMention>(&*conn)
            .map_err(Error::Database)?,
    ))
}
