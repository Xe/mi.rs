use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub preferred_username: Option<String>,
    pub avatar: Option<String>,
    pub admin: bool,
    pub banned: bool,
    pub published: String,
    pub updated: Option<String>,
    pub matrix_user_id: Option<String>,
    pub actor_id: String,
    pub bio: Option<String>,
    pub local: bool,
    pub banner: Option<String>,
    pub deleted: bool,
}
