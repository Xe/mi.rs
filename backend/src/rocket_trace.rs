use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::Header;
use rocket::{Data, Request, Response};
use rusty_ulid::generate_ulid_string;

pub struct RequestId;

impl Fairing for RequestId {
    fn info(&self) -> Info {
        Info {
            name: "Request ID",
            kind: Kind::Request | Kind::Response,
        }
    }

    fn on_request(&self, request: &mut Request, _: &Data) {
        match request.headers().get_one("X-Request-Id") {
            Some(_) => {}
            None => {
                let reqid = generate_ulid_string();
                request.add_header(Header::new("X-Request-Id", reqid.clone()));
            }
        };
    }

    fn on_response(&self, request: &Request, response: &mut Response) {
        match request.headers().get_one("X-Request-Id") {
            Some(reqid) => {
                response.set_header(Header::new("X-Request-Id", format!("{}", reqid)));
            }
            None => {}
        };
    }
}
