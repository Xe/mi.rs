/**
This module will fetch package tracking information from the OrangeConnex
API. I am not sure that this is completely kosher. Users of this function must
aggressively cache the results.
*/
use super::{Error, Result};
use serde::{Deserialize, Serialize};

/// An annoying wrapper type.
#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Info {
    pub has_business_exception: bool,
    pub success: bool,
    pub result: Wrapper,
}

impl Info {
    pub fn get_waybill(self) -> Option<Waybill> {
        if self.success == true && self.result.waybills.len() != 0 {
            Some(self.result.waybills[0].clone())
        } else {
            None
        }
    }
}

/// Another annoying wrapper type.
#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Wrapper {
    pub not_exists_tracking_numbers: Vec<String>,
    pub waybills: Vec<Waybill>,
}

/// The information about the most current status of a package, as well as its
/// source and destination.
#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Waybill {
    pub consignee_city_code: String,
    pub consignee_city_name: String,
    pub consignee_country_code: String,
    pub consignee_country_name: String,
    pub consignee_zip_code: String,
    pub consignment_city_code: String,
    pub consignment_city_name: String,
    pub consignment_country_code: String,
    pub consignment_country_name: String,
    pub consignment_zip_code: String,
    pub is_event_code: String,
    pub last_status: String,
    pub last_time: String,
    pub last_time_zone: String,
    pub last_timestamp: i64,
    pub last_zip_code: String,
    pub traces: Vec<Trace>,
}

/// Each step in the package's journey to being delivered.
#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Trace {
    pub event_desc: String,
    pub opr_city: Option<String>,
    pub opr_country: String,
    pub opr_time: String,
    pub opr_time_zone: String,
    pub opr_timestamp: i64,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
struct PostBody {
    tracking_numbers: Vec<String>,
}

/// Fetch tracking information from OrangeConnex.
pub fn get(tracking_number: String) -> Result<Info> {
    let resp = ureq::post(
        "https://azure-cn.orangeconnex.com/oc/capricorn-website/website/v1/tracking/traces",
    )
    .set(
        "User-Agent",
        "Mozilla/5.0 (X11; Linux x86_64; rv:84.0) Gecko/20100101 Firefox/84.0",
    )
    .set("Accept", "*/*")
    .set("Accept-Language", "en-US")
    .set("Content-Type", "application/json;charset=utf-8")
    .set("Origin", "https://www.orangeconnex.com")
    .set("Connection", "keep-alive")
    .set(
        "Referer",
        &format!(
            "https://www.orangeconnex.com/tracking?language=en&trackingnumber={}",
            tracking_number
        ),
    )
    .set("Cache-Control", "max-age=0")
    .send_json(serde_json::to_value(PostBody {
        tracking_numbers: vec![tracking_number],
    })?);

    if resp.ok() {
        Ok(resp.into_json_deserialize()?)
    } else {
        Err(match resp.synthetic_error() {
            Some(why) => Error::UReq(why.to_string()),
            None => Error::HttpStatus(resp.status()),
        })
    }
}
