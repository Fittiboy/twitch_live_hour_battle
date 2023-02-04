use crate::config::Token;
use crate::data_structures::*;
use reqwest::blocking;
use serde::Serialize;

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
