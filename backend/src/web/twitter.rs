use super::{Error, Result};
use rocket::fairing::AdHoc;
use twapi_ureq::*;

pub struct Client {
    consumer_token: String,
    consumer_secret: String,
    api_key: String,
    api_secret: String,
}

impl Client {
    pub fn fairing() -> AdHoc {
        AdHoc::on_attach("Twitter client", |rocket| {
            let cfg = rocket.config();
            let table = cfg.get_table("twitter").unwrap();
            let consumer_token = table["consumer_token"].as_str().unwrap().to_string();
            let consumer_secret = table["consumer_secret"].as_str().unwrap().to_string();
            let api_key = table["api_key"].as_str().unwrap().to_string();
            let api_secret = table["api_secret"].as_str().unwrap().to_string();

            Ok(rocket.manage(Client {
                consumer_token: consumer_token,
                consumer_secret: consumer_secret,
                api_key: api_key,
                api_secret: api_secret,
            }))
        })
    }

    pub fn tweet(&self, body: String) -> Result<()> {
        let url = "https://api.twitter.com/1.1/statuses/update.json";
        let form_options = vec![("status", body.as_str())];

        let resp = v1::post(
            url,
            &vec![],
            &form_options,
            &self.consumer_token,
            &self.consumer_secret,
            &self.api_key,
            &self.api_secret,
        );

        if resp.ok() {
            Ok(())
        } else {
            Err(match resp.synthetic_error() {
                Some(why) => Error::OldUReq(why.to_string()),
                None => Error::HttpStatus(resp.status()),
            })
        }
    }
}
