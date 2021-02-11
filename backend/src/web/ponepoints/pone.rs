use super::Client;
use crate::web::Result;
use serde::Deserialize;

impl Client {
    #[instrument(err, skip(self))]
    pub fn get_pone(&self, slug: String) -> Result<Pone> {
        Ok(
            ureq::get(&format!("https://points.horse/api/v1/pones/{}.json", slug))
                .set("Authorization", &format!("Api-Key {}", self.token.clone()))
                .set("User-Agent", crate::APPLICATION_NAME)
                .set("Accept", "application/json")
                .call()?
                .into_json()?,
        )
    }

    #[instrument(err, skip(self))]
    pub fn get_self(&self) -> Result<Pone> {
        Ok(ureq::get("https://points.horse/api/v1/pones/me.json")
            .set("Authorization", &format!("Api-Key {}", self.token.clone()))
            .set("User-Agent", crate::APPLICATION_NAME)
            .set("Accept", "application/json")
            .call()?
            .into_json()?)
    }

    #[instrument(err, skip(self))]
    pub fn list_pones(&self) -> Result<Vec<PoneData>> {
        Ok(ureq::get("https://points.horse/api/v1/pones.json")
            .set("Authorization", &format!("Api-Key {}", self.token.clone()))
            .set("User-Agent", crate::APPLICATION_NAME)
            .set("Accept", "application/json")
            .call()?
            .into_json::<PoneList>()?
            .unwrap())
    }
}

#[derive(Clone, Deserialize)]
struct PoneList {
    pones: Vec<PoneData>,
}

impl PoneList {
    fn unwrap(self) -> Vec<PoneData> {
        self.pones
    }
}

#[derive(Clone, Deserialize)]
pub struct Pone {
    pub pone: PoneData,
}

#[derive(Clone, Deserialize, Debug)]
pub struct PoneData {
    pub slug: String,
    pub avatar_url: Option<String>,
    pub joined_at: String,
    pub giftable_points_count: Option<i32>,
    pub daily_giftable_points_count: Option<i32>,
    pub links: PoneLinks,
    pub name: String,
    pub points_count: i32,
}

#[derive(Clone, Deserialize, Debug)]
pub struct PoneLinks {
    #[serde(rename = "self")]
    pub myself: String,
    pub page: String,
    pub achievements: String,
    pub points: String,
    pub granted_points: String,
    pub groups: String,
}

#[cfg(test)]
mod tests {
    use super::Pone;
    use serde_json::from_str;

    #[test]
    fn json_slug() {
        let data = include_str!("./testdata/pone_slug.json");
        let pone: Pone = from_str(data).expect("able to parse ./testdata/pone_slug.json");

        assert!(pone.pone.giftable_points_count.is_none());
        assert!(pone.pone.daily_giftable_points_count.is_none());
    }

    #[test]
    fn json_myself() {
        let data = include_str!("./testdata/pone_self.json");
        let pone: Pone = from_str(data).expect("able to parse ./testdata/pone_self.json");

        assert!(pone.pone.giftable_points_count.is_some());
        assert!(pone.pone.daily_giftable_points_count.is_some());
    }
}
