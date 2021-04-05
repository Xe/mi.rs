use askama::Template;
use rocket::{fairing::AdHoc, response::content::Css, Request};

#[derive(Template)]
#[template(path = "app.html")]
struct App {
    title: String,
    message: String,
}

#[get("/")]
fn frontend() -> App {
    App {
        title: "Mi".to_string(),
        message: "Loading...".to_string(),
    }
}

#[get("/login")]
fn login() -> App {
    frontend()
}

#[derive(Template)]
#[template(path = "notfound.html")]
struct NotFound {
    title: String,
    message: String,
}

#[catch(404)]
fn not_found(req: &Request) -> NotFound {
    NotFound {
        title: "Not found".to_string(),
        message: format!("{} not found", req.uri()),
    }
}

#[get("/static/gruvbox.css")]
fn gruvbox() -> Css<String> {
    Css(include_str!(env!("GRUVBOX_CSS")).to_string())
}

pub fn fairing() -> AdHoc {
    AdHoc::on_attach("frontend integration", |rocket| {
        Ok(rocket
            .register(catchers![not_found])
            .mount("/", routes![frontend, gruvbox, login]))
    })
}
