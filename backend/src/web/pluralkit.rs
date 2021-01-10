use super::{Error, Result};
use rocket::fairing::AdHoc;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Deserialize, Debug)]
pub struct ProxyTag {
    pub prefix: Option<String>,
    pub suffix: Option<String>,
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
    pub created: String,
}

#[derive(Serialize, Debug)]
pub struct SwitchRequest {
    pub members: Vec<String>,
}

#[derive(Deserialize, Debug)]
pub struct SystemStatus {
    pub timestamp: String,
    pub members: Vec<Member>,
}

pub struct Client {
    api_token: String,
    member_mappings: BTreeMap<String, String>,
}

impl Client {
    pub fn new(token: String) -> Client {
        Client {
            api_token: token,
            member_mappings: BTreeMap::new(),
        }
    }

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
    pub fn status(&self, system_id: String) -> Result<SystemStatus> {
        let resp = ureq::get(&format!(
            "https://api.pluralkit.me/v1/s/{}/fronters",
            system_id
        ))
        .set("Authorization", &self.api_token)
        .set("User-Agent", crate::APPLICATION_NAME)
        .call()
        .map_err(Error::UReq)?;

        Ok(resp.into_json()?)
    }

    #[instrument(err, skip(self))]
    pub fn switch(&self, member_name: String) -> Result {
        let member = self
            .member_mappings
            .get(&member_name)
            .ok_or_else(|| Error::SystemmateMappingNotFound(member_name.clone()))?
            .clone();

        ureq::post("https://api.pluralkit.me/v1/s/switches")
            .set("Authorization", &self.api_token)
            .set("User-Agent", crate::APPLICATION_NAME)
            .send_json(serde_json::to_value(SwitchRequest {
                members: vec![member],
            })?)
            .map_err(Error::UReq)?;

        Ok(())
    }
}
