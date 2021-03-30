use super::{Error, Result};
use crate::MainDatabase;
use rocket::http::{Cookie, Cookies};
use rocket_contrib::json::Json;
use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
pub struct Me {
    pub me: String,
}

#[get("/auth?<me>&<client_id>&<redirect_uri>&<state>&<response_type>")]
#[instrument(skip(db, cookies), err)]
pub fn auth(
    db: MainDatabase,
    me: String,
    client_id: String,
    redirect_uri: String,
    state: String,
    response_type: String,
    mut cookies: Cookies,
) -> Result {
    let _ = db;
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
#[instrument(skip(db, code), err)]
pub fn send_code(
    db: MainDatabase,
    client_id: String,
    redirect_uri: String,
    code: String,
) -> Result<Json<Me>> {
    let _ = db;
    let _ = code;
    Ok(Json(Me {
        me: "https://christine.website".to_string(),
    }))
}
