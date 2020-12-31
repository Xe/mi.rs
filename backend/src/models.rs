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

#[derive(Queryable, Associations, Insertable, Serialize, Clone)]
#[table_name = "webmentions"]
pub struct WebMention {
    pub id: String,
    pub source_url: String,
    pub target_url: String,
    pub title: Option<String>,
}

#[derive(AsChangeset)]
#[table_name = "webmentions"]
pub struct UpdateWebMentionSource {
    pub source_url: String,
}

#[derive(Queryable, Associations, Insertable)]
#[table_name = "blogposts"]
pub struct Blogpost {
    pub url: String,
    pub title: String,
}

#[derive(Queryable, Associations, Insertable, AsChangeset)]
#[table_name = "orangeconnex_packages"]
pub struct OrangeConnexPackage {
    pub tracking_number: String,
    pub recieved: bool,
}

#[derive(Queryable, Associations, Insertable, Serialize)]
#[table_name = "orangeconnex_traces"]
pub struct OrangeConnexTrace {
    pub id: String,
    pub tracking_number: String,
    pub description: String,
    pub city: Option<String>,
    pub country: String,
    pub time_recorded: String,
    pub time_zone: String,
    pub ts: i32,
}

impl OrangeConnexTrace {
    pub fn from_trace(t: crate::web::orange_connex::Trace, tracking_number: String) -> Self {
        use rusty_ulid::generate_ulid_string;
        Self {
            id: generate_ulid_string(),
            tracking_number: tracking_number,
            description: t.event_desc,
            city: t.opr_city,
            country: t.opr_country,
            time_recorded: t.opr_time,
            time_zone: t.opr_time_zone,
            ts: t.opr_timestamp,
        }
    }
}
