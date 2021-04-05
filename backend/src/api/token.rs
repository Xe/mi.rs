use super::{Error, Result};
use crate::{models, paseto, schema, MainDatabase};
use ::paseto::PasetoBuilder;
use chrono::prelude::*;
use diesel::prelude::*;
use rocket::State;
use rocket_contrib::json::Json;
use rusty_ulid::generate_ulid_string;

#[get("/token/info")]
pub fn info(tok: paseto::Token) -> Json<paseto::Token> {
    Json(tok)
}

#[post("/token/mint?<aud>&<sub>")]
#[instrument(skip(kp, conn), err)]
pub fn mint(
    conn: MainDatabase,
    tok: paseto::Token,
    kp: State<paseto::Keypair>,
    aud: String,
    sub: String,
) -> Result<String> {
    let kp = kp.inner().ed25519_keypair();
    let now = Utc::now();

    let tok = models::Token {
        id: generate_ulid_string(),
        sub: sub.clone(),
        aud: aud.clone(),
        iat: now.to_rfc3339(),
        iss: format!("{}", crate::APPLICATION_NAME),
        exp: None,
        valid: None,
    };

    diesel::insert_into(schema::tokens::table)
        .values(&tok)
        .execute(&*conn)
        .map_err(Error::Database)?;

    PasetoBuilder::new()
        .set_ed25519_key(kp)
        .set_issued_at(Some(now))
        .set_issuer(format!("api call from {}", tok.sub))
        .set_audience(aud)
        .set_jti(tok.id.clone())
        .set_subject(sub)
        .build()
        .map_err(|why| {
            error!("can't make paseto: {}", why);
            Error::PasetoCreationError(format!("{}", why))
        })
}
