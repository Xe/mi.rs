use super::{Error, Result};
use rocket::fairing::AdHoc;
use serde::{Deserialize, Serialize};

pub mod community;
pub mod post;
pub mod user;

#[derive(Serialize, Debug, Clone)]
pub struct Login {
    pub username_or_email: String,
    pub password: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct LoginResponse {
    pub jwt: String,
}

pub struct Client {
    community: i32,
    token: String,
}

impl Client {
    pub fn new(username: String, password: String, community: String) -> Result<Client> {
        let resp = ureq::post("https://lemmy.ml/api/v2/user/login")
            .set("User-Agent", crate::APPLICATION_NAME)
            .send_json(serde_json::to_value(Login {
                username_or_email: username,
                password,
            })?)?;

        let lr: LoginResponse = resp.into_json()?;

        let resp = ureq::get("https://lemmy.ml/api/v2/community")
            .set("User-Agent", crate::APPLICATION_NAME)
            .query("auth", &lr.jwt.clone())
            .query("name", &community)
            .call()?;
        let gcr: community::GetCommunityResponse = resp.into_json()?;

        Ok(Self {
            token: lr.jwt,
            community: gcr.community_view.community.id,
        })
    }

    pub fn fairing() -> AdHoc {
        AdHoc::on_attach("Lemmy client", |rocket| {
            let cfg = rocket.config();
            let table = cfg.get_table("lemmy").unwrap();
            let username = table["username"].as_str().unwrap().to_string();
            let password = table["password"].as_str().unwrap().to_string();
            let community = table["community"].as_str().unwrap().to_string();
            let cli = Client::new(username, password, community).unwrap();

            Ok(rocket.manage(cli))
        })
    }

    pub fn post(&self, url: String, title: String) -> Result<post::CreatePostResponse> {
        ureq::post("https://lemmy.ml/api/v2/post")
            .set("User-Agent", crate::APPLICATION_NAME)
            .send_json(serde_json::to_value(post::CreatePost {
                name: title,
                url: Some(url),
                body: None,
                nsfw: false,
                community_id: self.community.clone(),
                auth: self.token.clone(),
            })?)?
            .into_json()
            .map_err(Error::FuturesIO)
    }
}
