#[get("/.within/botinfo")]
fn botinfo() -> &'static str {
    include_str!("./botinfo.txt")
}

#[get("/robots.txt")]
fn robots() -> String {
    include_str!("./robots.txt").into_string()
}

#[get("/security.txt")]
fn security() -> String {
    include_str!("./security.txt").into_string()
}
