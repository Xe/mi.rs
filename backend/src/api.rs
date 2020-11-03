use crate::{models, schema, web, MainDatabase};
use chrono::prelude::*;
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
use rusty_ulid::generate_ulid_string;
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
        .map(|(switch, member)| FrontChange {
            duration: switch.duration(),
            id: switch.id,
            who: member.cmene,
            started_at: switch.started_at,
            ended_at: switch.ended_at,
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
        Some((switch, member)) => Ok(Json(FrontChange {
            duration: switch.duration(),
            id: switch.id,
            who: member.cmene,
            started_at: switch.started_at,
            ended_at: switch.ended_at,
        })),
        None => Err(Error::NotFound),
    }
}

#[post("/switches/switch", data = "<who>")]
#[instrument(skip(conn, sc, pk), err)]
pub fn make_switch(
    conn: MainDatabase,
    who: StringBody,
    sc: State<web::switchcounter::Client>,
    pk: State<web::pluralkit::Client>,
) -> Result<String> {
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
            })
            .execute(&*conn)
            .map_err(Error::Database)
    }?;

    diesel::insert_into(switches::table)
        .values(&switch)
        .execute(&*conn)
        .map_err(Error::Database)?;

    info!(from = &member.cmene[..], to = &to.cmene[..], "switched");

    sc.switch(to.cmene.clone())?;
    pk.switch(to.cmene.clone())?;

    Ok(to.cmene)
}

#[get("/switches/<switch_id>")]
#[instrument(skip(conn), err)]
pub fn get_switch(conn: MainDatabase, switch_id: String) -> Result<Json<FrontChange>> {
    use schema::{members, switches::dsl::switches};

    let (switch, member): (models::Switch, models::Member) = switches
        .find(switch_id)
        .inner_join(members::table)
        .get_result(&*conn)
        .map_err(Error::Database)?;

    Ok(Json(FrontChange {
        duration: switch.duration(),
        id: switch.id,
        who: member.cmene,
        started_at: switch.started_at,
        ended_at: switch.ended_at,
    }))
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
