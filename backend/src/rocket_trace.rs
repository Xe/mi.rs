use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::{ContentType, Header, Method, Status};
use rocket::{Data, Request, Response};
use rusty_ulid::generate_ulid_string;
use std::mem;
use tracing::{span, span::Entered, Level, Span};

struct SpanWrapper<'a> {
    span: Box<Span>,
    entered: Entered<'a>,
}

pub struct TraceRequest;

impl Fairing for TraceRequest {
    fn info(&self) -> Info {
        Info {
            name: "Tracing spans per request",
            kind: Kind::Request | Kind::Response,
        }
    }

    fn on_request(&self, request: &mut Request, _: &Data) {
        let reqid: String = match request.headers().get_one("X-Request-Id") {
            Some(reqid) => reqid.to_string(),
            None => {
                let reqid = generate_ulid_string();
                request.add_header(Header::new("X-Request-Id", reqid.clone()));
                reqid
            }
        };

        request.local_cache(|| {
            let span = span!(
                Level::INFO,
                "request",
                method = &request.method().to_string()[..],
                uri = &request.uri().to_string()[..],
                addr = &request
                    .client_ip()
                    .map(|ip| ip.to_string())
                    .unwrap_or("unknown".to_string())[..],
                request_id = &reqid[..],
            );
            let span = Box::new(span);
            let entered = span.enter();
            let entered_lt = unsafe { mem::transmute::<_, Entered<'static>>(entered) };

            Box::new(SpanWrapper {
                entered: entered_lt,
                span: span,
            })
        });
    }

    fn on_response(&self, request: &Request, response: &mut Response) {
        let span: &Box<SpanWrapper> = request.local_cache(|| {
            Box::new({
                let span = error_span!(
                    "somehow the span wasn't put into the request, neat",
                    method = &request.method().to_string()[..],
                    uri = &request.uri().to_string()[..],
                    address = &request
                        .real_ip()
                        .map(|ip| ip.to_string())
                        .unwrap_or("unknown".to_string())[..],
                );

                let span = Box::new(span);
                let entered = span.enter();
                let entered_lt = unsafe { mem::transmute::<_, Entered<'static>>(entered) };

                SpanWrapper {
                    entered: entered_lt,
                    span: span,
                }
            })
        });
        match request.headers().get_one("X-Request-Id") {
            Some(reqid) => {
                response.set_header(Header::new("X-Request-Id", format!("{}", reqid)));
            }
            None => {}
        };
        info!(
            status_code = &response.status().code,
            status_reason = response.status().reason
        );
        drop(span);
    }
}
