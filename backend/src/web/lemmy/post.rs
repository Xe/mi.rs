use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug, Clone)]
pub struct CreatePost {
    pub name: String,
    pub url: Option<String>,
    pub body: Option<String>,
    pub nsfw: bool,
    pub community_id: i32,
    pub auth: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct CreatePostResponse {
    pub post_view: PostView,
}

#[derive(Deserialize, Debug, Clone)]
pub struct PostView {
    pub post: Post,
    pub creator: super::user::User,
    pub community: super::community::Community,
    pub creator_banned_from_community: bool,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Post {
    pub id: i32,
    pub name: String,
    pub url: Option<String>,
    pub body: Option<String>,
    pub creator_id: i32,
    pub community_id: i32,
    pub removed: bool,
    pub locked: bool,
    pub published: String,
    pub updated: Option<String>,
    pub deleted: bool,
    pub nsfw: bool,
    pub stickied: bool,
    pub embed_title: Option<String>,
    pub embed_description: Option<String>,
    pub embed_html: Option<String>,
    pub thumbnail_url: Option<String>,
    pub ap_id: String,
    pub local: bool,
}
