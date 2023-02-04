use serde::Deserialize;
use serde_json::Number;
use std::path::PathBuf;

#[derive(Debug)]
pub struct Config {
    dir: PathBuf,
}

impl Config {
    pub fn new() -> Config {
        let mut dir = dirs::config_dir().expect("config dir should always exist");
        dir.extend(["twitch_live_hour_battle", "placeholder.txt"]);
        Config { dir }
    }

    fn file_from_dir(&mut self, filename: &str) -> String {
        self.select_file(filename);
        let file_contents = std::fs::read_to_string(&self.dir)
            .expect(&format!("{} should be in config directory", filename));
        file_contents.trim().to_owned()
    }

    fn select_file(&mut self, filename: &str) {
        self.dir.set_file_name(filename);
    }

    pub fn client_id(&mut self) -> String {
        self.file_from_dir("client_id.txt")
    }

    pub fn client_secret(&mut self) -> String {
        self.file_from_dir("client_secret.txt")
    }
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct Token {
    pub access_token: String,
    pub expires_in: Number,
    pub token_type: String,
}
