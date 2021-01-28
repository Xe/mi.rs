use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct GetCommunityResponse {
    pub community_view: CommunityView,
    pub online: i32,
}

#[derive(Deserialize, Debug, Clone)]
pub struct CommunityView {
    pub community: Community,
    pub creator: super::user::User,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Community {
    pub id: i32,
    pub name: String,
    pub title: String,
    pub description: Option<String>,
    pub category_id: i32,
    pub creator_id: i32,
    pub removed: bool,
    pub published: String,
    pub updated: Option<String>,
    pub deleted: bool,
    pub nsfw: bool,
    pub actor_id: String,
    pub local: bool,
    pub icon: Option<String>,
    pub banner: Option<String>,
}
