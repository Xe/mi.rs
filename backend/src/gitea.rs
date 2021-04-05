use crate::{
    api::{self, Result},
    models,
    schema::gitea_tokens,
    web::Error,
    Gitea, MainDatabase,
};
use diesel::prelude::*;
use rocket::{
    http::{Cookie, Cookies, SameSite},
    response::Redirect,
    State,
};
use rocket_oauth2::{OAuth2, TokenResponse};
use serde::{Deserialize, Serialize};

/// A user.
/// https://try.gitea.io/api/swagger#model-User
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct User {
    pub avatar_url: String,
    pub created: String,
    pub email: String,
    pub full_name: String,
    pub id: i64,
    pub is_admin: bool,
    pub language: String,
    pub last_login: String,
    pub login: String,
}

fn user(token: String) -> Result<User> {
    let resp = ureq::get("https://tulpa.dev/api/v1/user")
        .set("Authorization", &format!("bearer {}", token))
        .set("User-Agent", crate::APPLICATION_NAME)
        .call()
        .map_err(|why| Error::OldUReq(format!("{}", why)))?;
    let user: User = resp.into_json()?;
    Ok(user)
}

#[get("/")]
#[instrument(skip(oauth2, cookies))]
pub fn login(oauth2: OAuth2<Gitea>, mut cookies: Cookies<'_>) -> Redirect {
    oauth2.get_redirect(&mut cookies, &[""]).unwrap()
}

#[get("/callback")]
#[instrument(skip(conn, token, cookies, kp), err)]
pub fn callback(
    conn: MainDatabase,
    kp: State<crate::paseto::Keypair>,
    token: TokenResponse<Gitea>,
    mut cookies: Cookies<'_>,
) -> crate::web::Result<String> {
    let tok = token.access_token().to_string();
    let refresh = token.refresh_token().unwrap().to_string();

    let gitea_user = user(tok.clone()).map_err(|why| Error::OldUReq(format!("{}", why)))?;

    if !gitea_user.is_admin {
        return Err(Error::OldUReq("I'm sorry dave, but I can't do that".into()));
    }

    let tok = models::GiteaToken {
        id: rusty_ulid::generate_ulid_string(),
        user_id: gitea_user.id.to_string(),
        access_token: tok,
        refresh_token: refresh,
    };

    diesel::insert_into(gitea_tokens::table)
        .values(&tok)
        .execute(&*conn)
        .map_err(api::Error::Database)
        .map_err(|why| Error::OldUReq(format!("{}", why)))?;

    let tok = api::token::mint(
        conn,
        crate::paseto::Token::default(),
        kp,
        "https://christine.website".into(),
        format!("{}", gitea_user.full_name),
    )
    .map_err(|why| Error::OldUReq(format!("{}", why)))?;

    cookies.add(
        Cookie::build("token", tok.clone())
            .same_site(SameSite::Lax)
            .finish(),
    );

    Ok(tok)
}
