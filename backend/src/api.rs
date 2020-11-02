use crate::{models, schema, MainDatabase};
use chrono::prelude::*;
use diesel::prelude::*;
use rocket::{
    data::{self, FromDataSimple},
    http::Status,
    request::Request,
    response::Responder,
    Data,
    Outcome::*,
    Response,
};
use rocket_contrib::json::Json;
use rusty_ulid::generate_ulid_string;
use std::{convert::TryInto, io::Read};

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
    pub duration: Option<i32>,
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
pub fn make_switch(conn: MainDatabase, who: StringBody) -> Result<String> {
    use schema::{members, switches};
    let who = who.unwrap();

    let (last, member): (models::Switch, models::Member) = switches::table
        .inner_join(members::table)
        .order_by(switches::dsl::started_at.desc())
        .limit(1)
        .load(&*conn)
        .map_err(Error::Database)?
        .pop()
        .ok_or_else(|| Error::NotFound)?;

    let to: models::Member = members::dsl::members
        .filter({
            use members::dsl::cmene;
            cmene.eq(who)
        })
        .limit(1)
        .load::<models::Member>(&*conn)
        .map_err(Error::Database)?
        .pop()
        .ok_or_else(|| Error::NotFound)?;

    let now = Utc::now().naive_utc();

    let switch = models::NewSwitch {
        id: generate_ulid_string(),
        member_id: to.id,
        started_at: now,
    };

    {
        use schema::switches::dsl::*;
        diesel::update(switches.find(last.id))
            .set(&models::UpdateSwitchTime {
                ended_at: Some(now.clone()),
                duration: Some(
                    now.clone()
                        .signed_duration_since(last.started_at)
                        .num_seconds()
                        .try_into()
                        .expect("don't expect a switch to last 30+ years"),
                ),
            })
            .execute(&*conn)
            .map_err(Error::Database)
    }?;

    diesel::insert_into(switches::table)
        .values(&switch)
        .execute(&*conn)
        .map_err(Error::Database)?;

    info!(from = &member.cmene[..], to = &to.cmene[..], "switched");

    Ok(to.cmene)
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
