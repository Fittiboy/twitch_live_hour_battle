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
        self.token = self.get_twitch_oauth2_token();
        self.set_twitch_specific_headers();
    }

    fn get_twitch_oauth2_token(&self) -> String {
        let poster = self.get_auth_poster();
        let response = TwitchClient::post_for_token(poster);
        response.access_token
    }

    fn get_auth_poster(&self) -> blocking::RequestBuilder {
        let body = self.oauth2_body();
        self.post(OAUTH2_URL)
            .header(header::CONTENT_TYPE, "application/x-www-form-urlencoded")
            .body(body)
    }

    fn post_for_token(poster: blocking::RequestBuilder) -> Token {
        let response = TwitchClient::send_auth_poster(poster);
        TwitchClient::parse_auth_response(response)
    }

    fn send_auth_poster(poster: blocking::RequestBuilder) -> blocking::Response {
        poster
            .send()
            .expect("should be able to initiate oauth2 flow")
    }

    fn parse_auth_response(response: blocking::Response) -> Token {
        response
            .json::<Token>()
            .expect("should be able to parse oauth2 flow response")
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
        let schedule = self.videos(broadcaster_name);
        dbg!(&schedule);
        0
    }

    fn videos(&self, broadcaster_name: &str) -> Schedule {
        let query = self.videos_query_string(broadcaster_name);
        let res = self
            .get(VIDEOS_URL)
            .query(&[query])
            .send()
            .expect("should always be able to get schedule after acquiring broadcaster id");
        dbg!(&res);
        res.json::<Schedule>()
            .expect("should be able to parse correct schedule response")
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

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct Schedule {
    data: Vec<Segment>,
    broadcaster_id: String,
    broadcaster_login: String,
    vacation: Value,
    pagination: Pagination,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct Segment {
    id: String,
    start_time: String,
    end_time: String,
    title: String,
    canceled_until: String,
    category: Category,
    is_recurring: bool,
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
