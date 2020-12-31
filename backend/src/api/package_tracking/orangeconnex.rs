use crate::{
    api::{Error, Result, StringBody},
    models, paseto, schema, MainDatabase,
};
use diesel::prelude::*;
use rocket_contrib::json::Json;

#[post("/packages/orangeconnex/track", data = "<tn>")]
#[instrument(skip(conn), err)]
pub fn track(conn: MainDatabase, tn: StringBody, tok: paseto::Token) -> Result<String> {
    use schema::orangeconnex_packages::table;
    let tn = tn.unwrap();

    diesel::insert_into(table)
        .values(&models::OrangeConnexPackage {
            tracking_number: tn.clone(),
            recieved: false,
        })
        .execute(&*conn)
        .map_err(Error::Database)?;

    Ok(format!("now tracking package {}", tn))
}

#[get("/packages/orangeconnex")]
#[instrument(skip(conn), err)]
pub fn list(
    conn: MainDatabase,
    tok: paseto::Token,
) -> Result<Json<Vec<models::OrangeConnexPackage>>> {
    use schema::orangeconnex_packages;

    Ok(Json(
        orangeconnex_packages::table
            .load::<models::OrangeConnexPackage>(&*conn)
            .map_err(Error::Database)?,
    ))
}

#[get("/packages/orangeconnex/status?<tn>")]
#[instrument(skip(conn), err)]
pub fn status(
    conn: MainDatabase,
    tn: String,
    tok: paseto::Token,
) -> Result<Json<Vec<models::OrangeConnexTrace>>> {
    use schema::orangeconnex_traces::dsl::*;

    Ok(Json(
        orangeconnex_traces
            .filter(tracking_number.eq(tn))
            .load::<models::OrangeConnexTrace>(&*conn)
            .map_err(Error::Database)?,
    ))
}

#[post("/packages/orangeconnex/delivered?<tn>")]
#[instrument(skip(conn), err)]
pub fn recieved(conn: MainDatabase, tn: String, tok: paseto::Token) -> Result<String> {
    use schema::orangeconnex_packages::dsl::*;

    diesel::update(orangeconnex_packages.find(tn.clone()))
        .set(&models::OrangeConnexPackage {
            tracking_number: tn.clone(),
            recieved: true,
        })
        .execute(&*conn)
        .map_err(Error::Database)?;

    Ok(format!("{} is marked as recieved", tn))
}
