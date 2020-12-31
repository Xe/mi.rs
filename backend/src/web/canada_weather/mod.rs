use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

pub mod types;

/// The credit string for this data.
///
/// XXX(acli): the license [here](https://dd.weather.gc.ca/doc/LICENCE_GENERAL.txt)
/// demands that we include this string somewhere in data derived from this API.
/// This must be manually included in each response to remain within the scope
/// of the license.
pub const DATA_SOURCE: &'static str = "Data Source: Environment and Climate Change Canada";

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Report {
    pub data_source: String,
    pub location: Location,
    pub conditions: Conditions,
    pub forecast: Vec<Forecast>,
}

impl From<types::SiteData> for Report {
    fn from(sd: types::SiteData) -> Self {
        let forecast: Vec<Forecast> = sd
            .forecast_group
            .forecast
            .into_iter()
            .map(Into::into)
            .collect();
        Report {
            data_source: DATA_SOURCE.to_string(),
            location: sd.location.into(),
            conditions: sd.current_conditions.into(),
            forecast: forecast,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Location {
    pub lat: String,
    pub lon: String,
    pub code: String,
    pub name: String,
    pub region: String,
}

impl From<types::Location> for Location {
    fn from(l: types::Location) -> Self {
        Location {
            lat: l.name.lat.unwrap(),
            lon: l.name.lon.unwrap(),
            code: l.name.code,
            name: l.name.name,
            region: l.region,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Conditions {
    pub as_of_utc: NaiveDateTime,
    pub as_of_local: NaiveDateTime,
    pub condition: String,
    pub temperature: f64,
    pub dewpoint: f64,
    pub windchill: Option<f64>,
    /// This is in kilo-Pascals
    pub pressure: f64,
    pub humidity: f64,
    pub icon_url: String,
}

impl From<types::CurrentConditions> for Conditions {
    fn from(cc: types::CurrentConditions) -> Self {
        let dt: Vec<NaiveDateTime> = cc.date_time.clone().into_iter().map(Into::into).collect();
        Conditions {
            as_of_utc: dt[0],
            as_of_local: dt[1],
            condition: cc.condition,
            temperature: cc.temperature.value.unwrap(),
            dewpoint: cc.dewpoint.value.unwrap(),
            windchill: match cc.wind_chill {
                None => None,
                Some(wc) => Some(wc.value.unwrap()),
            },
            pressure: cc.pressure.value.unwrap(),
            humidity: cc.relative_humidity.value.unwrap(),
            icon_url: format!(
                "https://weather.gc.ca/weathericons/{}.{}",
                cc.icon_code.value, cc.icon_code.format
            ),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Forecast {
    pub period: String,
    pub summary: String,
    pub icon_url: String,
    pub temp_summary: String,
    pub high: Option<f64>,
    pub low: Option<f64>,
    pub humidity: f64,
}

impl From<types::Forecast> for Forecast {
    fn from(f: types::Forecast) -> Self {
        Self {
            period: f.period.value,
            summary: f.text_summary,
            icon_url: format!(
                "https://weather.gc.ca/weathericons/{}.{}",
                f.abbreviated_forecast.icon_code.value, f.abbreviated_forecast.icon_code.format
            ),
            temp_summary: f.temperatures.text_summary,
            high: if f.temperatures.temperature.class.as_ref().unwrap() == "high" {
                f.temperatures.temperature.value
            } else {
                None
            },
            low: if f.temperatures.temperature.class.unwrap() == "low" {
                f.temperatures.temperature.value
            } else {
                None
            },
            humidity: f.relative_humidity.value.unwrap(),
        }
    }
}
