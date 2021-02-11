#[macro_use]
extern crate tracing;

#[cfg(feature = "mi")]
use rocket::fairing::AdHoc;

mod points;
mod pone;

pub use points::*;
pub use pone::*;

pub(crate) use mi_web::APPLICATION_NAME;

pub struct Client {
    token: String,
    #[allow(dead_code)]
    validation: String,
}

impl Client {
    pub fn new(token: String, validation: String) -> Self {
        Self { token, validation }
    }

    #[cfg(feature = "mi")]
    pub fn fairing() -> AdHoc {
        AdHoc::on_attach("pone points", |rocket| {
            let cfg = rocket.config();
            let table = cfg.get_table("ponepoints").unwrap();
            let token = table["token"].as_str().unwrap().to_string();
            let validation = table["validation"].as_str().unwrap().to_string();
            let cli = Self::new(token, validation);

            Ok(rocket.manage(cli))
        })
    }
}
