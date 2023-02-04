use twitch_live_hour_battle::authenticated_twitch_client;

fn main() {
    let client = authenticated_twitch_client();
    let yvonnie_hours = client.total_hours("yvonnie");
    let kkatamina_hours = client.total_hours("kkatamina");
    println!("Yvonne:  {}h\nMiyoung: {}h", yvonnie_hours, kkatamina_hours);
}
