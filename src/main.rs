use twitch_live_hour_battle::get_app_access_client;

fn main() {
    let client = get_app_access_client();
    let yvonnie_hours = client.total_hours("yvonnie");
    let kkatamina_hours = client.total_hours("kkatamina");
    println!("Yvonne:  {}h\nMiyoung: {}h", yvonnie_hours, kkatamina_hours);
}
