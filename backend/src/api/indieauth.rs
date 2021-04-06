use super::{Error, Result};
use crate::{models, paseto::Token, schema, MainDatabase};
use askama::Template;
use diesel::prelude::*;
use rocket::response::Redirect;
use rocket_contrib::json::Json;
use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
pub struct Me {
    pub me: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_token: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
}

#[get("/auth?<me>&<client_id>&<redirect_uri>&<state>&<response_type>")]
#[instrument(skip(db), err)]
pub fn auth(
    tok: Token,
    db: MainDatabase,
    me: String,
    client_id: String,
    redirect_uri: String,
    state: String,
    response_type: String,
) -> Result<Authz> {
    let _ = db;
    match response_type.as_str() {
        "code" | "id" => {}
        _ => return Err(Error::WrongIndieAuthResponseType(response_type)),
    };

    if me.as_str() != "https://christine.website" {
        return Err(Error::NotFound);
    }

    let code = rusty_ulid::generate_ulid_string();

    use schema::indieauth_codes::dsl::indieauth_codes;
    diesel::insert_into(indieauth_codes)
        .values(&models::IndieauthCode {
            code: code.clone(),
            client_id: client_id.clone(),
            redirect_uri: redirect_uri.clone(),
            state: state.clone(),
            response_type: response_type.clone(),
            authorized: false,
        })
        .execute(&*db)
        .map_err(Error::Database)?;

    Ok(Authz {
        client_id,
        code,
        me,
    })
}

#[get("/auth/authorized?<code>")]
#[instrument(skip(db), err)]
pub fn authorized(tok: Token, db: MainDatabase, code: String) -> Result<Redirect> {
    use schema::indieauth_codes::dsl::indieauth_codes;

    let iac: models::IndieauthCode = indieauth_codes
        .find(&code)
        .get_result(&*db)
        .map_err(Error::Database)?;

    diesel::update(indieauth_codes.find(&iac.code))
        .set(&models::UpdateIndieauthCodeAuthorized { authorized: true })
        .execute(&*db)
        .map_err(Error::Database)?;

    if iac.code != code {
        return Err(Error::NotFound);
    }

    let u =
        url::Url::parse_with_params(&iac.redirect_uri, &[("code", &code), ("state", &iac.state)])
            .map_err(|_| Error::NotFound)?;

    Ok(Redirect::to(u.to_string()))
}

#[derive(Template)]
#[template(path = "authz.html")]
pub struct Authz {
    client_id: String,
    me: String,
    code: String,
}

#[get("/auth?<code>&<redirect_uri>&<client_id>", rank = 2)]
#[instrument(skip(db, code), err)]
pub fn send_code(
    db: MainDatabase,
    client_id: String,
    redirect_uri: String,
    code: String,
) -> Result<Json<Me>> {
    use schema::indieauth_codes::dsl;

    let iac: models::IndieauthCode = dsl::indieauth_codes
        .find(&code)
        .get_result(&*db)
        .map_err(Error::Database)?;

    if !iac.authorized {
        return Err(Error::NotFound);
    }

    diesel::delete(dsl::indieauth_codes.filter(dsl::code.eq(code)))
        .execute(&*db)
        .map_err(Error::Database)?;

    Ok(Json(Me {
        me: "https://christine.website".to_string(),
        access_token: None,
        scope: None,
    }))
}
