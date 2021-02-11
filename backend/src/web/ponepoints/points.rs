use super::Client;
use crate::web::Result;
use serde::Deserialize;

impl Client {
    #[instrument(err, skip(self))]
    pub fn get_point_granted_for(&self, slug: String) -> Result<Vec<PointGrant>> {
        Ok(ureq::get(&format!(
            "https://points.horse/api/v1/pones/{}/points.json",
            slug
        ))
        .set("Authorization", &format!("Api-Key {}", self.token.clone()))
        .set("User-Agent", crate::APPLICATION_NAME)
        .set("Accept", "application/json")
        .call()?
        .into_json::<PointList>()?
        .unwrap())
    }

    #[instrument(err, skip(self))]
    pub fn get_point_grants_for(&self, slug: String) -> Result<Vec<PointGrant>> {
        Ok(ureq::get(&format!(
            "https://points.horse/api/v1/pones/{}/granted.json",
            slug
        ))
        .set("Authorization", &format!("Api-Key {}", self.token.clone()))
        .set("User-Agent", crate::APPLICATION_NAME)
        .set("Accept", "application/json")
        .call()?
        .into_json::<PointList>()?
        .unwrap())
    }

    #[instrument(err, skip(self))]
    pub fn get_grant_details(&self, pone_slug: String, point_id: String) -> Result<PointGrant> {
        Ok(ureq::get(&format!(
            "https://points.horse/api/v1/pones/{}/points/{}.json",
            pone_slug, point_id
        ))
        .set("Authorization", &format!("Api-Key {}", self.token.clone()))
        .set("User-Agent", crate::APPLICATION_NAME)
        .set("Accept", "application/json")
        .call()?
        .into_json::<PointWrapper>()?
        .unwrap())
    }
}

#[derive(Clone, Deserialize)]
pub struct PointGrant {
    pub id: i32,
    pub count: i32,
    pub granted_at: String,
    pub links: PointLinks,
    pub message: String,
}

#[derive(Clone, Deserialize)]
pub struct PointLinks {
    #[serde(rename = "self")]
    pub myself: String,
    pub pone: String,
    pub granted_by: String,
}

#[derive(Clone, Deserialize)]
struct PointList {
    points: Vec<PointGrant>,
}

impl PointList {
    pub fn unwrap(self) -> Vec<PointGrant> {
        self.points
    }
}

#[derive(Clone, Deserialize)]
struct PointWrapper {
    point: PointGrant,
}

impl PointWrapper {
    pub fn unwrap(self) -> PointGrant {
        self.point
    }
}

#[cfg(test)]
mod tests {
    use super::PointList;
    use serde_json::from_str;

    #[test]
    fn json_point_grants_slug() {
        let _pl: PointList = from_str(include_str!("./testdata/pone_grants_slug.json"))
            .expect("to parse pone_grants_slug.json");
    }
}
