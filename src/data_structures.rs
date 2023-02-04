use serde::Deserialize;
use serde_json::{Number, Value};

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct Videos {
    pub data: Vec<Video>,
    pub pagination: Pagination,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct Video {
    pub id: String,
    pub stream_id: String,
    pub user_id: String,
    pub user_login: String,
    pub user_name: String,
    pub title: String,
    pub description: String,
    pub created_at: String,
    pub published_at: String,
    pub url: String,
    pub thumbnail_url: String,
    pub viewable: String,
    pub view_count: Number,
    pub language: String,
    pub r#type: String,
    pub duration: String,
    pub muted_segments: Option<Value>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct Category {
    pub id: String,
    pub name: String,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct Pagination {
    pub cursor: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Users {
    pub data: Vec<User>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct User {
    pub id: String,
    pub login: String,
    pub display_name: String,
    pub r#type: String,
    pub broadcaster_type: String,
    pub description: String,
    pub profile_image_url: String,
    pub offline_image_url: String,
    pub view_count: Option<Number>,
    pub email: Option<String>,
    pub created_at: String,
}