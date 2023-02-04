use reqwest::{blocking, header, IntoUrl};
use serde::Deserialize;
use serde_json::{Number, Value};
use std::path::PathBuf;

const OAUTH2_URL: &str = "https://id.twitch.tv/oauth2/token";
const VIDEOS_URL: &str = "https://api.twitch.tv/helix/videos";
const USERS_URL: &str = "https://api.twitch.tv/helix/users";

fn main() {
    let client = get_app_access_client();
    let yvonnie_hours = client.total_hours("yvonnie");
    let kkatamina_hours = client.total_hours("kkatamina");
    println!("Yvonne:  {}h\nMiyoung: {}h", yvonnie_hours, kkatamina_hours);
}

fn get_app_access_client() -> TwitchClient {
    let mut client = TwitchClient::new();
    client.authenticate();
    client
}

#[derive(Debug)]
struct TwitchClient {
    reqwest_client: reqwest::blocking::Client,
    client_id: String,
    client_secret: String,
    token: String,
    headers: header::HeaderMap,
}

impl TwitchClient {
    fn new() -> TwitchClient {
        let mut config = Config::new();
        TwitchClient {
            reqwest_client: blocking::Client::new(),
            client_id: config.client_id(),
            client_secret: config.client_secret(),
            token: "".to_owned(),
            headers: header::HeaderMap::new(),
        }
    }

    fn authenticate(&mut self) {
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
        Poster(
            self.post(OAUTH2_URL)
                .header(header::CONTENT_TYPE, "application/x-www-form-urlencoded")
                .body(body),
        )
    }

    fn set_twitch_specific_headers(&mut self) {
        let mut headers = header::HeaderMap::new();
        let auth_header = self.auth_header();
        let client_id_header = self.client_id_header();
        headers.insert(header::AUTHORIZATION, auth_header);
        headers.insert("Client-Id", client_id_header);

        self.headers = headers;
    }

    fn oauth2_body(&self) -> String {
        format!(
            "client_id={}&client_secret={}&grant_type=client_credentials",
            self.client_id, self.client_secret
        )
    }

    fn post<U: IntoUrl>(&self, url: U) -> blocking::RequestBuilder {
        self.reqwest_client.post(url).headers(self.headers.clone())
    }

    fn total_hours(&self, broadcaster_name: &str) -> u32 {
        let videos = self.videos(broadcaster_name);
        dbg!(&videos);
        0
    }

    fn videos(&self, broadcaster_name: &str) -> Videos {
        self.videos_getter(broadcaster_name).videos()
    }

    fn videos_getter(&self, broadcaster_name: &str) -> Getter {
        let query = self.videos_query_string(broadcaster_name);
        Getter(self.get(VIDEOS_URL).query(&[query]))
    }

    fn videos_query_string(&self, broadcaster_name: &str) -> (String, String) {
        let broadcaster_id = self.broadcaster_id(broadcaster_name);
        ("user_id".to_owned(), broadcaster_id)
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

#[derive(Debug)]
struct Config {
    dir: PathBuf,
}

impl Config {
    fn new() -> Config {
        let mut dir = dirs::config_dir().expect("config dir should always exist");
        dir.extend(["twitch_live_hour_battle", "placeholder.txt"]);
        Config { dir }
    }

    fn file_from_dir(&mut self, filename: &str) -> String {
        self.dir.set_file_name(filename);
        let file_contents = std::fs::read_to_string(&self.dir)
            .expect(&format!("{} should be in config directory", filename));
        file_contents.trim().to_owned()
    }

    fn client_id(&mut self) -> String {
        self.file_from_dir("client_id.txt")
    }

    fn client_secret(&mut self) -> String {
        self.file_from_dir("client_secret.txt")
    }
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct Token {
    access_token: String,
    expires_in: Number,
    token_type: String,
}

#[derive(Debug)]
struct Poster(blocking::RequestBuilder);

impl Poster {
    fn post_for_token(self) -> Token {
        let response = self.send_auth_poster();
        Poster::parse_auth_response(response)
    }

    fn send_auth_poster(self) -> blocking::Response {
        self.0
            .send()
            .expect("should be able to initiate oauth2 flow")
    }

    fn parse_auth_response(response: blocking::Response) -> Token {
        response
            .json::<Token>()
            .expect("should be able to parse oauth2 flow response")
    }
}

#[derive(Debug)]
struct Getter(blocking::RequestBuilder);

impl Getter {
    fn videos(self) -> Videos {
        let response = self.get_videos();
        Getter::parse_videos_response(response)
    }

    fn get_videos(self) -> blocking::Response {
        self.0
            .send()
            .expect("should always be able to get schedule after acquiring broadcaster id")
    }

    fn parse_videos_response(response: blocking::Response) -> Videos {
        response
            .json::<Videos>()
            .expect("should be able to parse correct schedule response")
    }
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct Videos {
    data: Vec<Video>,
    pagination: Pagination,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct Video {
    id: String,
    stream_id: String,
    user_id: String,
    user_login: String,
    user_name: String,
    title: String,
    description: String,
    created_at: String,
    published_at: String,
    url: String,
    thumbnail_url: String,
    viewable: String,
    view_count: Number,
    language: String,
    r#type: String,
    duration: String,
    muted_segments: Option<Value>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct Category {
    id: String,
    name: String,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct Pagination {
    cursor: Option<String>,
}

#[derive(Debug, Deserialize)]
struct Users {
    data: Vec<User>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct User {
    id: String,
    login: String,
    display_name: String,
    r#type: String,
    broadcaster_type: String,
    description: String,
    profile_image_url: String,
    offline_image_url: String,
    view_count: Option<Number>,
    email: Option<String>,
    created_at: String,
}
