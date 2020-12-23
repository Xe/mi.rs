use chrono::{FixedOffset, NaiveDate, NaiveDateTime, TimeZone};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SiteData {
    pub license: String,
    pub date_time: Vec<DateTime>,
    pub location: Location,
    pub warnings: Option<Warnings>,
    pub current_conditions: CurrentConditions,
    pub forecast_group: ForecastGroup,
    // TODO(acli): hourly forecasts are not implemented yet.
    pub yesterday_conditions: Yesterday,
    pub rise_set: RiseSet,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Name {
    pub name: String,
    #[serde(rename = "$value")]
    pub value: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DateTime {
    pub name: String,
    pub zone: String,
    #[serde(rename = "UTCOffset")]
    pub utc_offset: i32,
    pub year: i32,
    pub month: Name,
    pub day: Name,
    pub hour: u32,
    pub minute: u32,
    pub time_stamp: String,
    pub text_summary: String,
}

impl Into<NaiveDateTime> for DateTime {
    fn into(self) -> NaiveDateTime {
        NaiveDate::from_ymd(self.year, self.month.value, self.day.value).and_hms(
            self.hour,
            self.minute,
            0,
        )
    }
}

impl Into<chrono::DateTime<FixedOffset>> for DateTime {
    fn into(self) -> chrono::DateTime<FixedOffset> {
        let hour = 3600;
        FixedOffset::east(self.utc_offset * hour)
            .ymd(self.year, self.month.value, self.day.value)
            .and_hms(self.hour, self.minute, 0)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Location {
    pub continent: String,
    pub country: CodeName,
    pub province: CodeName,
    pub name: CodeName,
    pub region: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CodeName {
    pub code: String,
    pub lat: Option<String>,
    pub lon: Option<String>,
    #[serde(rename = "$value")]
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct IconCode {
    pub format: String,
    #[serde(rename = "$value")]
    pub value: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MetricWithUnits {
    pub units: Option<String>,
    pub unit_type: Option<String>,
    pub change: Option<f64>,
    pub tendency: Option<String>,
    pub class: Option<String>,
    #[serde(rename = "$value")]
    pub value: Option<f64>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Wind {
    pub speed: MetricWithUnits,
    pub gust: MetricWithUnits,
    pub direction: String,
    pub bearing: MetricWithUnits,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CurrentConditions {
    pub station: CodeName,
    pub date_time: Vec<DateTime>,
    pub condition: String,
    pub icon_code: IconCode,
    pub temperature: MetricWithUnits,
    pub dewpoint: MetricWithUnits,
    pub wind_chill: Option<MetricWithUnits>,
    pub pressure: MetricWithUnits,
    pub visibility: MetricWithUnits,
    pub relative_humidity: MetricWithUnits,
    pub wind: Option<Wind>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Warnings {
    pub url: String,
    pub event: Vec<Event>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Event {
    pub r#type: String,
    pub priority: String,
    pub description: String,
    pub date_time: Vec<DateTime>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ForecastGroup {
    pub date_time: Vec<DateTime>,
    pub regional_normals: RegionalNormals,
    pub forecast: Vec<Forecast>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RegionalNormals {
    pub text_summary: String,
    pub temperature: Vec<MetricWithUnits>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Period {
    pub text_forecast_name: String,
    #[serde(rename = "$value")]
    pub value: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CloudPrecip {
    pub text_summary: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Forecast {
    pub period: Period,
    pub text_summary: String,
    pub cloud_precip: Option<CloudPrecip>,
    pub abbreviated_forecast: AbbreviatedForecast,
    pub temperatures: Temperatures,
    pub winds: Winds,
    pub precipitation: Option<Precipitation>,
    pub uv: Option<UVIndex>,
    pub relative_humidity: MetricWithUnits,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AbbreviatedForecast {
    pub icon_code: IconCode,
    pub pop: MetricWithUnits,
    pub text_summary: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Temperatures {
    pub text_summary: String,
    pub temperature: MetricWithUnits,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Winds {
    pub text_summary: Option<String>,
    pub wind: Option<Vec<Wind>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PrecipitationType {
    pub start: String,
    pub end: String,
    #[serde(rename = "$value")]
    pub value: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Accumulation {
    pub name: String,
    pub amount: MetricWithUnits,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Precipitation {
    pub text_summary: Option<String>,
    pub precip_type: Vec<PrecipitationType>,
    pub accumulation: Option<Accumulation>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Visibility {
    pub cause: String,
    pub text_summary: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UVIndex {
    pub category: String,
    pub index: String,
    pub text_summary: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Yesterday {
    pub temperature: Vec<MetricWithUnits>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RiseSet {
    pub disclaimer: String,
    pub date_time: Vec<DateTime>,
}
