use super::{Error, Result};
use crate::{
    models, paseto, schema,
    web::{self, discord_webhook::Client as DiscordWebhook, Error as WebError},
    MainDatabase,
};
use diesel::prelude::*;
use readability_fork::extractor::{self, Product};
use rocket::{
    http::Status,
    request::Form,
    response::{self, Responder},
    Request, Response, State,
};
use rocket_contrib::json::Json;
use rusty_ulid::generate_ulid_string;
use serde::Serialize;
use url::Url;

#[derive(FromForm, Debug, Serialize)]
pub struct WebMention {
    source: String,
    target: String,
    title: Option<String>,
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

    fn extract(&self) -> Result<Product> {
        let resp = ureq::get(&self.source)
            .set("User-Agent", crate::APPLICATION_NAME)
            .set("Mi-Mentioned-Url", &self.target)
            .call()?;

        let body = resp
            .into_string()
            .map_err(|why| Error::Web(web::Error::FuturesIO(why)))?;
        Ok(extractor::extract(
            &mut body.as_bytes(),
            &url::Url::parse(&self.source)?,
        )?)
    }
}

impl Into<models::WebMention> for WebMention {
    fn into(self) -> models::WebMention {
        models::WebMention {
            id: generate_ulid_string(),
            source_url: self.source,
            target_url: self.target,
            title: self.title,
        }
    }
}

impl Into<WebMention> for models::WebMention {
    fn into(self) -> WebMention {
        WebMention {
            source: self.source_url,
            target: self.target_url,
            title: self.title,
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
            .status(Status::Created)
            .ok()
    }
}

#[get("/webmention/for?<target>")]
#[instrument(skip(conn), err)]
pub fn lookup_target(conn: MainDatabase, target: String) -> Result<Json<Vec<WebMention>>> {
    use schema::webmentions::dsl::*;

    Ok(Json(
        webmentions
            .filter(target_url.eq(target))
            .load::<models::WebMention>(&*conn)
            .map_err(Error::Database)?
            .into_iter()
            .map(Into::into)
            .collect::<Vec<WebMention>>(),
    ))
}

#[post("/webmention/accept", data = "<mention>")]
#[instrument(skip(conn, dw), err)]
pub fn accept(
    conn: MainDatabase,
    mention: Form<WebMention>,
    dw: State<DiscordWebhook>,
) -> Result<models::WebMention> {
    use schema::webmentions;

    let mut mention = mention.into_inner();
    mention.check()?;
    mention
        .extract()
        .map_err(|why| {
            error!(
                "error extracting information from {}: {}",
                mention.source, why
            );

            why
        })
        .iter_mut()
        .for_each(|info| mention.title = Some(info.title.clone()));

    info!(
        source = &mention.source[..],
        target = &mention.target[..],
        "webmention received: {:?}",
        mention.title,
    );

    let wm: models::WebMention = mention.into();
    diesel::insert_into(webmentions::table)
        .values(&wm)
        .execute(&*conn)
        .map_err(Error::Database)?;

    bridgy_expand(conn, wm.clone())?;

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

pub fn bridgy_expand(conn: MainDatabase, wm: models::WebMention) -> Result {
    use crate::web::bridgy::parse;
    use schema::webmentions::dsl::*;

    if !wm.source_url.contains("https://brid-gy.appspot.com") {
        return Ok(());
    }

    if wm.source_url.contains("like/twitter") {
        return Ok(());
    }

    let resp = ureq::get(&wm.source_url)
        .set("User-Agent", crate::APPLICATION_NAME)
        .set("Mi-Mentioned-Url", &wm.target_url)
        .call()
        .map_err(WebError::UReq)?;

    let body: String = resp.into_string()?;
    let result = parse(&body).unwrap().unwrap();

    diesel::update(webmentions.find(wm.id))
        .set(&models::UpdateWebMentionSource {
            source_url: result.target,
        })
        .execute(&*conn)
        .map_err(Error::Database)
        .unwrap();
    Ok(())
}
