use super::{Error, Result};
use chrono::NaiveDateTime;
use rocket::fairing::AdHoc;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Deserialize, Debug)]
pub struct ProxyTag {
    pub prefix: String,
    pub suffix: String,
}

#[derive(Deserialize, Debug)]
pub struct Member {
    pub id: String,
    pub name: Option<String>,
    pub display_name: Option<String>,
    pub description: Option<String>,
    pub color: Option<String>,
    pub avatar_url: Option<String>,
    pub birthday: Option<String>,
    pub proxy_tags: Option<Vec<ProxyTag>>,
    pub keep_proxy: bool,
    pub created: NaiveDateTime,
}

#[derive(Serialize, Debug)]
pub struct SwitchRequest {
    pub members: Vec<String>,
}

pub struct Client {
    api_token: String,
    member_mappings: BTreeMap<String, String>,
}

impl Client {
    pub fn fairing() -> AdHoc {
        AdHoc::on_attach("PluralKit client", |rocket| {
            let cfg = rocket.config();
            let table = cfg.get_table("pluralkit").unwrap();
            let api_token = table["token"].as_str().unwrap().to_string();

            let mut member_mappings = BTreeMap::new();

            for (key, value) in table["mappings"].as_table().unwrap().iter() {
                member_mappings.insert(key.clone(), value.as_str().unwrap().to_string());
            }

            let cli = Client {
                api_token: api_token,
                member_mappings: member_mappings,
            };
            Ok(rocket.manage(cli))
        })
    }

    #[instrument(err, skip(self))]
    pub fn switch(&self, member_name: String) -> Result {
        let member = self
            .member_mappings
            .get(&member_name)
            .ok_or_else(|| Error::SystemmateMappingNotFound(member_name.clone()))?
            .clone();

        let resp = ureq::post("https://api.pluralkit.me/v1/s/switches")
            .set("Authorization", &self.api_token)
            .set("User-Agent", crate::APPLICATION_NAME)
            .send_json(serde_json::to_value(SwitchRequest {
                members: vec![member],
            })?);

        if resp.ok() {
            Ok(())
        } else {
            Err(match resp.synthetic_error() {
                Some(why) => Error::UReq(why.to_string()),
                None => Error::HttpStatus(resp.status()),
            })
        }
    }
}
