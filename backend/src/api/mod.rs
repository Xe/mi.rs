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

pub mod indieauth;
pub mod package_tracking;
pub mod posse;
pub mod switch;
pub mod token;
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

    #[error("can't switch to the same fronter {0}")]
    SameFronter(String),

    #[error("target doesn't mention source")]
    TargetDoesntMentionSource(String),

    #[error("readability error: {0}")]
    Readability(#[from] readability_fork::error::Error),

    #[error("ureq error: {0}")]
    UReq(#[from] ureq::Error),

    #[error("futures error: {0}")]
    Futures(#[from] futures_io::Error),

    #[error("paseto creation error: {0}")]
    PasetoCreationError(String),

    #[error("paseto validation error: {0}")]
    PasetoValidationError(String),

    #[error("no paseto in request")]
    NoPasetoInRequest,

    #[error("wrong indieauth response type: {0}")]
    WrongIndieAuthResponseType(String),

    #[error("json error: {0}")]
    JsonError(#[from] serde_json::Error),
}

pub type Result<T = ()> = std::result::Result<T, Error>;

impl<'a> Responder<'a> for Error {
    fn respond_to(self, _: &Request) -> ::std::result::Result<Response<'a>, Status> {
        error!("{}", self);
        match self {
            Error::NotFound => Err(Status::NotFound),
            Error::InvalidWebMention(_)
            | Error::SameFronter(_)
            | Error::WrongIndieAuthResponseType(_) => Err(Status::BadRequest),
            _ => Err(Status::InternalServerError),
        }
    }
}
