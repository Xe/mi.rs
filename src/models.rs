#[derive(Queryable)]
pub struct Member {
    pub id: i32,
    pub cmene: String,
    pub picurl: String,
}

#[derive(Queryable)]
pub struct Switch {
    pub id: String,
    pub who: String,
    pub started_at: String,
    pub ended_at: Option<String>,
    pub duration: String,
}
