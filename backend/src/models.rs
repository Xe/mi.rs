use crate::schema::*;
use chrono::NaiveDateTime;
use serde::Serialize;
use std::convert::TryInto;

#[derive(Queryable, Debug, Serialize)]
pub struct Member {
    pub id: i32,
    #[serde(rename = "name")]
    pub cmene: String,
    pub picurl: String,
}

#[derive(Queryable, Associations, Insertable)]
#[belongs_to(Member)]
#[table_name = "switches"]
pub struct Switch {
    pub id: String,
    pub member_id: i32,
    pub started_at: NaiveDateTime,
    pub ended_at: Option<NaiveDateTime>,
}

impl Switch {
    pub fn duration(&self) -> Option<i32> {
        match self.ended_at {
            None => None,
            Some(end_time) => Some(
                end_time
                    .clone()
                    .signed_duration_since(self.started_at)
                    .num_seconds()
                    .try_into()
                    .expect("don't expect a switch to last 30+ years"),
            ),
        }
    }
}

#[derive(Insertable)]
#[table_name = "switches"]
pub struct NewSwitch {
    pub id: String,
    pub member_id: i32,
    pub started_at: NaiveDateTime,
}

#[derive(AsChangeset)]
#[table_name = "switches"]
pub struct UpdateSwitchTime {
    pub ended_at: Option<NaiveDateTime>,
}

#[derive(Queryable, Associations, Insertable, Serialize)]
#[table_name = "webmentions"]
pub struct WebMention {
    pub id: String,
    pub source_url: String,
    pub target_url: String,
}
