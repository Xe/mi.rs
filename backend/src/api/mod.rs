use crate::{models, paseto, schema, web, MainDatabase};
use diesel::prelude::*;
use rocket::{
    data::{self, FromDataSimple},
    http::Status,
    request::Request,
    response::Responder,
    Data,
    Outcome::*,
    Response, State,
};
use rocket_contrib::json::Json;
use std::io::Read;

pub mod posse;
pub mod switch;
pub mod webmention;

#[get("/members")]
#[instrument(skip(conn), err)]
pub fn get_members(tok: paseto::Token, conn: MainDatabase) -> Result<Json<Vec<models::Member>>> {
    use schema::members;
    let results = members::table
        .load::<models::Member>(&*conn)
        .map_err(Error::Database)?;

    Ok(Json(results))
}

#[get("/token/info")]
pub fn token_info(tok: paseto::Token) -> Json<paseto::Token> {
    Json(tok)
}

#[post("/tweet", data = "<body>")]
#[instrument(skip(tw), err)]
pub fn tweet(body: StringBody, tw: State<web::twitter::Client>, tok: paseto::Token) -> Result {
    tw.tweet(body.unwrap())?;

    Ok(())
}

#[post("/toot", data = "<body>")]
#[instrument(skip(ma), err)]
pub fn toot(body: StringBody, ma: State<web::mastodon::Client>, tok: paseto::Token) -> Result {
    ma.toot(body.unwrap())?;

    Ok(())
}

#[derive(Debug)]
pub struct StringBody(String);

impl StringBody {
    fn unwrap(self) -> String {
        self.0
    }
}

impl FromDataSimple for StringBody {
    type Error = String;

    fn from_data(_req: &Request, data: Data) -> data::Outcome<Self, String> {
        let mut contents = String::new();

        if let Err(e) = data.open().take(256).read_to_string(&mut contents) {
            return Failure((Status::InternalServerError, format!("{:?}", e)));
        }

        Success(StringBody(contents))
    }
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("internal database error: {0}")]
    Database(#[from] diesel::result::Error),

    #[error("not found")]
    NotFound,

    #[error("web API interop error: {0}")]
    Web(#[from] web::Error),

    #[error("URL parsing error: {0}")]
    URL(#[from] url::ParseError),

    #[error("invalid webmention: {0}")]
    InvalidWebMention(String),

    #[error("can't switch to the same fronter")]
    SameFronter,
}

pub type Result<T = ()> = std::result::Result<T, Error>;

impl<'a> Responder<'a> for Error {
    fn respond_to(self, _: &Request) -> ::std::result::Result<Response<'a>, Status> {
        error!("{}", self);
        match self {
            Error::NotFound => Err(Status::NotFound),
            Error::InvalidWebMention(_) | Error::SameFronter => Err(Status::BadRequest),
            _ => Err(Status::InternalServerError),
        }
    }
}
