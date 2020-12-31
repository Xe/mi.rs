#[macro_use]
extern crate tracing;

use color_eyre::eyre::Result;
use diesel::prelude::*;

use mi::{api::Error, establish_connection, models, schema, web::orange_connex};

fn main() -> Result<()> {
    color_eyre::install()?;
    tracing_subscriber::fmt::init();

    info!(
        "{} package tracking updater starting up",
        mi::APPLICATION_NAME
    );
    let conn = establish_connection();

    // OrangeConnex package tracking
    {
        use schema::{
            orangeconnex_packages::dsl::*, orangeconnex_traces::dsl::orangeconnex_traces,
        };
        let packages = orangeconnex_packages
            .filter(recieved.eq(false))
            .load::<models::OrangeConnexPackage>(&conn)
            .map_err(Error::Database)?;

        for package in packages.into_iter() {
            let info = orange_connex::get(package.tracking_number.clone())?
                .get_waybill()
                .unwrap();
            let tn = package.tracking_number;

            let traces: Vec<models::OrangeConnexTrace> = info
                .traces
                .into_iter()
                .map(|tr| models::OrangeConnexTrace::from_trace(tr, tn.clone()))
                .collect();

            let _ = diesel::insert_into(orangeconnex_traces)
                .values(&traces)
                //.on_conflict((time_recorded, time_zone, ts))
                //.do_nothing()
                .execute(&conn)
                .map_err(Error::Database);
        }
    }

    Ok(())
}
