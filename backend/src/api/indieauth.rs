use super::{Error, Result};
use crate::{
    paseto::{Keypair, Token},
    MainDatabase,
};
use paseto::tokens::{validate_public_token, PasetoPublicKey};
use rocket::{
    http::{Cookie, Cookies},
    State,
};
use rocket_contrib::json::Json;
use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
pub struct Me {
    pub me: String,
}

#[get("/auth?<me>&<client_id>&<redirect_uri>&<state>&<response_type>")]
#[instrument(skip(db, cookies, kp), err)]
pub fn auth(
    db: MainDatabase,
    me: String,
    client_id: String,
    redirect_uri: String,
    state: String,
    response_type: String,
    kp: State<Keypair>,
    mut cookies: Cookies,
) -> Result {
    match response_type.as_str() {
        "code" | "id" => {}
        _ => return Err(Error::WrongIndieAuthResponseType(response_type)),
    };

    cookies.add_private(Cookie::new("redirect_uri", redirect_uri));
    cookies.add_private(Cookie::new("state", state));
    cookies.add_private(Cookie::new("me", me));

    Ok(())
}

#[get("/auth?<code>&<redirect_uri>&<client_id>")]
#[instrument(skip(db, cookies, paseto_key, code), err)]
pub fn send_code(
    db: MainDatabase,
    client_id: String,
    redirect_uri: String,
    code: String,
    paseto_key: State<PasetoPublicKey>,
    mut cookies: Cookies,
) -> Result<Json<Me>> {
    let val = validate_public_token(&code, None, &*paseto_key)
        .map_err(|why| Error::PasetoValidationError(format!("{}", why)))?;

    let tok: Token = serde_json::from_value(val)?;

    Ok(Json(Me {
        me: cookies
            .get_private("me")
            .ok_or(Error::NotFound)?
            .value()
            .to_string(),
    }))
}
