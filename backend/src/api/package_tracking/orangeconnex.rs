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

#[get("/packages/orangeconnex/status?<tn>")]
#[instrument(skip(conn), err)]
pub fn status(
    conn: MainDatabase,
    tn: String,
    tok: paseto::Token,
) -> Result<Json<Vec<models::OrangeConnexTrace>>> {
    use schema::orangeconnex_traces;

    Ok(Json(
        orangeconnex_traces::table
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
