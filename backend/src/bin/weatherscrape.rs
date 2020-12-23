#[macro_use]
extern crate tracing;

use color_eyre::eyre::Result;
use encoding_rs::WINDOWS_1252;
use encoding_rs_io::DecodeReaderBytesBuilder;
use serde_xml_rs::from_reader;

use mi::*;

pub const WEATHER_URL: &'static str =
    "https://dd.weather.gc.ca/citypage_weather/xml/QC/s0000635_e.xml";

fn main() -> Result<()> {
    color_eyre::install()?;
    tracing_subscriber::fmt::init();

    info!("{} weather importer starting up", mi::APPLICATION_NAME);

    let resp = ureq::get(WEATHER_URL).set("User-Agent", WEATHER_URL).call();

    if !resp.ok() {
        panic!(
            "{}",
            match resp.synthetic_error() {
                Some(why) => why.to_string(),
                None => resp.status_line().to_string(),
            }
        );
    }

    let fin = DecodeReaderBytesBuilder::new()
        .encoding(Some(WINDOWS_1252))
        .build(resp.into_reader());
    let data: web::canada_weather::SiteData = from_reader(fin)?;

    println!("{:#?}", data);

    Ok(())
}
