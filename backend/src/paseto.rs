use paseto::tokens::{validate_public_token, PasetoPublicKey};
use paseto::PasetoBuilder;
use ring::signature::Ed25519KeyPair;
use rocket::{
    fairing::AdHoc,
    http::Status,
    request::{self, FromRequest, Request},
    Outcome, State,
};
use rusty_ulid::generate_ulid_string;
use serde::{Deserialize, Serialize};

pub fn ed25519_keypair() -> AdHoc {
    AdHoc::on_attach("ed25519 keypair for paseto", |rocket| {
        let cfg = rocket.config();
        let table = cfg.get_table("paseto").unwrap();
        let private = table["private"].as_str().unwrap().to_string();
        let private = hex::decode(&private).unwrap();
        let public = table["public"].as_str().unwrap().to_string();
        let public = hex::decode(&public).unwrap();
        let kp = Ed25519KeyPair::from_seed_and_public_key(&private, &public).unwrap();

        let token = PasetoBuilder::new()
            .set_ed25519_key(kp)
            .set_issued_at(None)
            .set_issuer("manual API call".into())
            .set_audience("wizards".into())
            .set_jti(generate_ulid_string())
            .set_subject("Within".into())
            .build()
            .unwrap();
        debug!("token: {}", token);

        Ok(rocket.manage(PasetoPublicKey::ED25519KeyPair(
            Ed25519KeyPair::from_seed_and_public_key(&private, &public).unwrap(),
        )))
    })
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Token {
    pub jti: String,
    pub sub: String,
    pub aud: String,
    pub iss: String,
}

impl<'a, 'r> FromRequest<'a, 'r> for Token {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        let keys: Vec<_> = request.headers().get("authorization").collect();
        match keys.len() {
            1 => {
                let tok = keys[0];
                let paseto_key = request.guard::<State<PasetoPublicKey>>().unwrap();

                match validate_public_token(tok, None, &paseto_key) {
                    Ok(val) => {
                        let tok: Token = serde_json::from_value(val).unwrap();
                        info!(id = &tok.jti[..], "token used");
                        Outcome::Success(tok)
                    }
                    Err(why) => {
                        error!("paseto error: {}", why);
                        Outcome::Failure((Status::Unauthorized, ()))
                    }
                }
            }
            _ => Outcome::Failure((Status::Unauthorized, ())),
        }
    }
}
