use super::{Error, Result, StringBody};
use crate::{
    models, paseto, schema,
    web::{PluralKit, SwitchCounter},
    MainDatabase,
};
use chrono::prelude::*;
use diesel::prelude::*;
use rocket::State;
use rocket_contrib::json::Json;
use rusty_ulid::generate_ulid_string;

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
pub fn list(
    conn: MainDatabase,
    count: Option<i64>,
    page: Option<i64>,
    tok: paseto::Token,
) -> Result<Json<Vec<FrontChange>>> {
    use schema::{members, switches};

    let count = count.unwrap_or(30);
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
pub fn current_front(conn: MainDatabase, tok: paseto::Token) -> Result<Json<FrontChange>> {
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

#[get("/switches/current/text")]
#[instrument(skip(conn), err)]
pub fn current_front_text(conn: MainDatabase, tok: paseto::Token) -> Result<String> {
    use schema::{members, switches};

    let mut front: Vec<(models::Switch, models::Member)> = switches::table
        .inner_join(members::table)
        .order_by(switches::dsl::started_at.desc())
        .limit(1)
        .load(&*conn)
        .map_err(Error::Database)?;

    match front.pop() {
        Some((_, member)) => Ok(member.cmene),
        None => Err(Error::NotFound),
    }
}

#[post("/switches/switch", data = "<who>")]
#[instrument(skip(conn, sc, pk), err)]
pub fn switch(
    conn: MainDatabase,
    who: StringBody,
    sc: State<SwitchCounter>,
    pk: State<PluralKit>,
    tok: paseto::Token,
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

    if member.cmene == to.cmene {
        return Err(Error::SameFronter(member.cmene));
    }

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
pub fn get(tok: paseto::Token, conn: MainDatabase, switch_id: String) -> Result<Json<FrontChange>> {
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
