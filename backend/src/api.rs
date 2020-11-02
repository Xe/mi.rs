use crate::{models, schema, MainDatabase};
use chrono::NaiveDateTime;
use diesel::prelude::*;
use rocket::{
    data::{self, FromDataSimple},
    http::{ContentType, Status},
    request::{self, FromRequest, Request},
    response::Responder,
    Data,
    Outcome::*,
    Response,
};
use rocket_contrib::json::Json;
use std::io::Read;

#[get("/members")]
#[instrument(skip(conn), err)]
pub fn get_members(conn: MainDatabase) -> Result<Json<Vec<models::Member>>> {
    use schema::members;
    let results = members::table
        .load::<models::Member>(&*conn)
        .map_err(Error::Database)?;

    Ok(Json(results))
}

#[derive(serde::Serialize)]
pub struct FrontChange {
    pub id: String,
    pub who: String, // models::Member.name
    pub started_at: NaiveDateTime,
    pub ended_at: Option<NaiveDateTime>,
    pub duration: i32,
}

#[get("/switches?<count>&<page>")]
#[instrument(skip(conn), err)]
pub fn get_switches(
    conn: MainDatabase,
    count: Option<i64>,
    page: Option<i64>,
) -> Result<Json<Vec<FrontChange>>> {
    use schema::{members, switches};

    let count = count.unwrap_or(50);
    let page = page.unwrap_or(0);

    let count = if count < 100 { count } else { 100 };

    let result: Vec<FrontChange> = switches::table
        .inner_join(members::table)
        .order_by(switches::dsl::started_at.desc())
        .limit(count)
        .offset(count * (page - 1))
        .load::<(models::Switch, models::Member)>(&*conn)
        .map_err(Error::Database)?
        .into_iter()
        .map(|front| FrontChange {
            id: front.0.id,
            who: front.1.cmene,
            started_at: front.0.started_at,
            ended_at: front.0.ended_at,
            duration: front.0.duration,
        })
        .collect();

    match result.len() {
        0 => Err(Error::NotFound),
        _ => Ok(Json(result)),
    }
}

#[get("/switches/current")]
#[instrument(skip(conn), err)]
pub fn get_current_front(conn: MainDatabase) -> Result<Json<FrontChange>> {
    use schema::{members, switches};

    let mut front: Vec<(models::Switch, models::Member)> = switches::table
        .inner_join(members::table)
        .order_by(switches::dsl::started_at.desc())
        .limit(1)
        .load(&*conn)
        .map_err(Error::Database)?;

    match front.pop() {
        Some(front) => Ok(Json(FrontChange {
            id: front.0.id,
            who: front.1.cmene,
            started_at: front.0.started_at,
            ended_at: front.0.ended_at,
            duration: front.0.duration,
        })),
        None => Err(Error::NotFound),
    }
}

#[post("/switches/switch", data = "<who>")]
#[instrument(skip(conn), err)]
pub fn make_switch(conn: MainDatabase, who: StringBody) -> Result {
    info!("got here");
    Ok(())
}

#[derive(Debug)]
pub struct StringBody(String);

impl FromDataSimple for StringBody {
    type Error = String;

    fn from_data(req: &Request, data: Data) -> data::Outcome<Self, String> {
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
}

pub type Result<T = ()> = std::result::Result<T, Error>;

impl<'a> Responder<'a> for Error {
    fn respond_to(self, _: &Request) -> ::std::result::Result<Response<'a>, Status> {
        error!("{}", self);
        match self {
            Error::NotFound => Err(Status::NotFound),
            _ => Err(Status::InternalServerError),
        }
    }
}
