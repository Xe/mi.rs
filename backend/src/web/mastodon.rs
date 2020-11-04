use super::{Error, Result};
use rocket::fairing::AdHoc;

pub struct Client {
    instance_url: String,
    token: String,
    account_name: String,
}

impl Client {
    pub fn fairing() -> AdHoc {
        AdHoc::on_attach("Mastodon client", |rocket| {
            let cfg = rocket.config();
            let table = cfg.get_table("mastodon").unwrap();
            let instance_url = table["instance"].as_str().unwrap().to_string();
            let token = table["token"].as_str().unwrap().to_string();
            let account_name = table["account"].as_str().unwrap().to_string();

            let cli = Client {
                instance_url: instance_url,
                token: token,
                account_name: account_name,
            };

            Ok(rocket.manage(cli))
        })
    }

    pub fn account_name(&self) -> String {
        self.account_name.clone()
    }

    pub fn toot(&self, body: String) -> Result {
        let url = &format!("{}/api/v1/statuses", self.instance_url);

        let resp = ureq::post(url)
            .set("Authorization", &format!("bearer {}", self.token))
            .set("User-Agent", crate::APPLICATION_NAME)
            .send_form(&[("status", body.as_str())]);

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
