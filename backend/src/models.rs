use crate::schema::*;
use chrono::NaiveDateTime;
use serde::Serialize;

#[derive(Queryable, Debug, Serialize)]
pub struct Member {
    pub id: i32,
    #[serde(rename = "name")]
    pub cmene: String,
    pub picurl: String,
}

#[derive(Queryable, Associations)]
#[belongs_to(Member)]
#[table_name = "switches"]
pub struct Switch {
    pub id: String,
    pub member_id: i32,
    pub started_at: NaiveDateTime,
    pub ended_at: Option<NaiveDateTime>,
    pub duration: i32,
}
