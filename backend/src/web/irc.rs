use super::{Error, Result};
use rocket::fairing::AdHoc;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Clone, Debug, Deserialize)]
pub struct Body {
    content: String,
}

impl Body {
    pub fn new<T>(body: T) -> Body
    where
        T: Into<String>,
    {
        Body {
            content: body.into(),
        }
    }
}

pub struct Client {
    url: String,
}

impl Client {
    pub fn new(url: String) -> Self {
        Client { url: url }
    }

    pub fn fairing() -> AdHoc {
        AdHoc::on_attach("IRC webhook", |rocket| {
            let webhook_url = rocket.config().get_string("irc_webhook").unwrap();
            Ok(rocket.manage(Client::new(webhook_url)))
        })
    }

    #[instrument(skip(self), err)]
    pub fn send(&self, body: String) -> Result<()> {
        let body = body.replace("\n", " - ");

        ureq::post(&self.url)
            .set("User-Agent", crate::APPLICATION_NAME)
            .send_json(serde_json::to_value(Body::new(body))?)
            .map_err(Error::UReq)?;

        Ok(())
    }
}
