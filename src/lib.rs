use crate::config::Config;
use crate::data_structures::{Users, Videos};
use crate::requests::{Getter, OAuth2Body, Poster, VideosQuery};
use reqwest::{blocking, header, IntoUrl};

mod config;
mod data_structures;
mod requests;

const OAUTH2_URL: &str = "https://id.twitch.tv/oauth2/token";
const VIDEOS_URL: &str = "https://api.twitch.tv/helix/videos";
const USERS_URL: &str = "https://api.twitch.tv/helix/users";

pub fn authenticated_twitch_client() -> TwitchClient {
    let mut client = TwitchClient::new();
    client.authenticate();
    client
}

#[derive(Debug)]
pub struct TwitchClient {
    reqwest_client: reqwest::blocking::Client,
    client_id: String,
    client_secret: String,
    token: String,
    headers: header::HeaderMap,
}

impl TwitchClient {
    pub fn new() -> TwitchClient {
        let mut config = Config::new();
        TwitchClient {
            reqwest_client: blocking::Client::new(),
            client_id: config.client_id(),
            client_secret: config.client_secret(),
            token: "".to_owned(),
            headers: header::HeaderMap::new(),
        }
    }

    pub fn authenticate(&mut self) {
        self.token = self.twitch_oauth2_token();
        self.set_twitch_specific_headers();
    }

    fn twitch_oauth2_token(&self) -> String {
        let poster = self.auth_poster();
        let response = poster.post_for_token();
        response.access_token
    }

    fn auth_poster(&self) -> Poster {
        let body = self.oauth2_body();
        Poster(self.post(OAUTH2_URL).form(&body))
    }

    fn set_twitch_specific_headers(&mut self) {
        let mut headers = header::HeaderMap::new();
        headers.insert(header::AUTHORIZATION, self.auth_header());
        headers.insert("Client-Id", self.client_id_header());
        self.headers = headers;
    }

    fn oauth2_body(&self) -> OAuth2Body {
        OAuth2Body {
            client_id: self.client_id.clone(),
            client_secret: self.client_secret.clone(),
            grant_type: "client_credentials".to_owned(),
        }
    }

    fn post<U: IntoUrl>(&self, url: U) -> blocking::RequestBuilder {
        self.reqwest_client.post(url).headers(self.headers.clone())
    }

    pub fn total_hours(&self, broadcaster_name: &str) -> u32 {
        let videos = self.videos(broadcaster_name);
        dbg!(&videos);
        0
    }

    fn videos(&self, broadcaster_name: &str) -> Videos {
        self.videos_getter(broadcaster_name).videos()
    }

    fn videos_getter(&self, broadcaster_name: &str) -> Getter {
        let query = self.videos_query(broadcaster_name);
        Getter(self.get(VIDEOS_URL).query(&query))
    }

    fn videos_query(&self, broadcaster_name: &str) -> VideosQuery {
        VideosQuery {
            user_id: self.broadcaster_id(broadcaster_name),
            // "archive" represents just actual VODs of past streams
            r#type: "archive".to_owned(),
            // 100 is the maximum accepted value
            first: "100".to_owned(),
        }
    }

    fn broadcaster_id(&self, broadcaster_name: &str) -> String {
        let query = self.id_query_string(broadcaster_name);
        let response = self
            .get(USERS_URL)
            .query(&[query])
            .send()
            .expect("should be able to get user ID for broadcaster login")
            .json::<Users>()
            .expect("should be able to parse user ID response");
        TwitchClient::broadcaster_id_from_api_response(response)
    }

    fn broadcaster_id_from_api_response(response: Users) -> String {
        let user = response.data.into_iter().next().expect(
            "if the login given was correct, there should always be one user in the response",
        );
        user.id
    }

    fn id_query_string(&self, broadcaster_name: &str) -> (String, String) {
        ("login".to_owned(), broadcaster_name.to_string())
    }

    fn get<U: IntoUrl>(&self, url: U) -> blocking::RequestBuilder {
        self.reqwest_client.get(url).headers(self.headers.clone())
    }

    fn auth_header(&self) -> header::HeaderValue {
        format!("Bearer {}", self.token)
            .parse()
            .expect("it should always be possible to parse this header")
    }

    fn client_id_header(&self) -> header::HeaderValue {
        self.client_id
            .parse()
            .expect("it should always be possible to parse this header")
    }
}
