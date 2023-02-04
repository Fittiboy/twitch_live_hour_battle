use crate::config::Config;
use crate::twitchclient::TwitchClient;

pub const OAUTH2_URL: &str = "https://id.twitch.tv/oauth2/token";
pub const VIDEOS_URL: &str = "https://api.twitch.tv/helix/videos";
pub const USERS_URL: &str = "https://api.twitch.tv/helix/users";

pub fn get_app_access_client() -> TwitchClient {
    let mut client = TwitchClient::new();
    client.authenticate();
    client
}

pub mod config;
pub mod requests;
pub mod twitchclient;
