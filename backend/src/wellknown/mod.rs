#[get("/.within/botinfo")]
pub fn botinfo() -> &'static str {
    include_str!("./botinfo.txt")
}

#[get("/robots.txt")]
pub fn robots() -> String {
    include_str!("./robots.txt").to_string()
}

#[get("/security.txt")]
pub fn security() -> String {
    include_str!("./security.txt").to_string()
}
