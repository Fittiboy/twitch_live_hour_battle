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
    token: String,
}

impl TwitchClient {
    fn new() -> TwitchClient {
        TwitchClient {
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
