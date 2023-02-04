use crate::config::Token;
use reqwest::blocking;
use serde::{Deserialize, Serialize};
use serde_json::{Number, Value};

#[derive(Debug)]
pub struct Poster(pub blocking::RequestBuilder);

impl Poster {
    pub fn post_for_token(self) -> Token {
        let response = self.send_auth_poster();
        Poster::parse_auth_response(response)
    }

    pub fn send_auth_poster(self) -> blocking::Response {
        self.0
            .send()
            .expect("should be able to initiate oauth2 flow")
    }

    pub fn parse_auth_response(response: blocking::Response) -> Token {
        response
            .json::<Token>()
            .expect("should be able to parse oauth2 flow response")
    }
}

#[derive(Debug, Serialize)]
pub struct OAuth2Body {
    pub client_id: String,
    pub client_secret: String,
    pub grant_type: String,
}

#[derive(Debug)]
pub struct Getter(pub blocking::RequestBuilder);

impl Getter {
    pub fn videos(self) -> Videos {
        let response = self.get_videos();
        Getter::parse_videos_response(response)
    }

    pub fn get_videos(self) -> blocking::Response {
        self.0
            .send()
            .expect("should always be able to get schedule after acquiring broadcaster id")
    }

    pub fn parse_videos_response(response: blocking::Response) -> Videos {
        response
            .json::<Videos>()
            .expect("should be able to parse correct schedule response")
    }
}

#[derive(Debug, Serialize)]
pub struct VideosQuery {
    pub user_id: String,
    pub r#type: String,
    pub first: String,
}

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
