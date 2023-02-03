use std::path::PathBuf;

fn main() {
    let client = get_app_access_client();
    let yvonnie_hours = client.get_total_hours("yvonnie");
    let kkatamina_hours = client.get_total_hours("kkatamina");
    println!("Yvonne:  {}h\nMiyoung: {}h", yvonnie_hours, kkatamina_hours);
}

fn get_app_access_client() -> TwitchClient {
    let mut client = TwitchClient::new();
    client.authenticate();
    client
}

struct TwitchClient {
    client_id: String,
    client_secret: String,
    token: String,
}

impl TwitchClient {
    fn new() -> TwitchClient {
        let mut config = Config::new();
        let client_id = config.get_client_id();
        let client_secret = config.get_client_secret();
        TwitchClient {
            client_id,
            client_secret,
            token: "".to_owned(),
        }
    }

    fn authenticate(&mut self) {
        self.token = "".to_owned();
    }

    fn get_total_hours(&self, broadcaster_name: &str) -> u32 {
        0
    }
}

struct Config {
    dir: PathBuf,
}

impl Config {
    fn new() -> Config {
        let mut dir = dirs::config_dir().expect("config dir should always exist");
        dir.extend(["twitch_live_hour_battle", "placeholder.txt"]);
        Config { dir }
    }

    fn get_file_from_dir(&mut self, filename: &str) -> String {
        self.dir.set_file_name(filename);
        std::fs::read_to_string(&self.dir)
            .expect(&format!("{} should be in config directory", filename))
    }

    fn get_client_id(&mut self) -> String {
        self.get_file_from_dir("client_id.txt")
    }

    fn get_client_secret(&mut self) -> String {
        self.get_file_from_dir("client_secret.txt")
    }
}
