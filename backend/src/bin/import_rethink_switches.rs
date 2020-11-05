#[macro_use]
extern crate tracing;

use chrono::prelude::*;
use color_eyre::eyre::Result;
use diesel::{prelude::*, SqliteConnection};
use serde::Deserialize;
use serde_json::from_reader;
use std::{env, fs::File, io::BufReader};

#[derive(Deserialize, Debug)]
struct RethinkTime {
    epoch_time: f64,
}

impl Into<NaiveDateTime> for RethinkTime {
    fn into(self) -> NaiveDateTime {
        NaiveDateTime::from_timestamp(self.epoch_time.round() as i64, 0)
    }
}

#[derive(Deserialize, Debug)]
struct RethinkRow {
    id: String,
    who: String,
    started_at: RethinkTime,
    ended_at: Option<RethinkTime>,
}

fn main() -> Result<()> {
    color_eyre::install()?;
    tracing_subscriber::fmt::init();

    info!("{} rethink dump importer starting up", mi::APPLICATION_NAME);

    let conn = establish_connection();

    let fname = env::args()
        .skip(1)
        .next()
        .expect("usage: import_rethink_switches </path/to/switches.json>");

    let fin = File::open(&fname)?;
    let bufreader = BufReader::new(fin);
    let data: Vec<mi::models::Switch> = from_reader::<BufReader<File>, Vec<RethinkRow>>(bufreader)?
        .into_iter()
        .map(|rr| mi::models::Switch {
            id: rr.id,
            member_id: member_to_id(rr.who),
            started_at: rr.started_at.into(),
            ended_at: rr.ended_at.map(|time| time.into()),
        })
        .collect::<Vec<mi::models::Switch>>();
    diesel::insert_into(mi::schema::switches::table)
        .values(&data)
        .execute(&conn)?;

    Ok(())
}

pub fn establish_connection() -> SqliteConnection {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub fn member_to_id(name: String) -> i32 {
    match name.as_str() {
        "Cadey" => 0,
        "Nicole" => 1,
        "Jessie" => 2,
        "Ashe" => 3,
        "Sephie" => 4,
        "Mai" => 5,

        _ => panic!("name not matched"),
    }
}
