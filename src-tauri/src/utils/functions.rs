use std::time::UNIX_EPOCH;

pub async fn get_name(uuid: &str) -> String {
    let url = format!(
        "https://sessionserver.mojang.com/session/minecraft/profile/{}",
        uuid
    );

    let response = reqwest::get(url).await.unwrap().text().await.unwrap();

    let json: serde_json::Value = serde_json::from_str(&response).unwrap();
    let name = json["name"].as_str().unwrap();

    name.to_string()
}

pub fn get_epoch() -> u128 {
    let now = std::time::SystemTime::now();
    let epoch = now.duration_since(UNIX_EPOCH).expect("Time went backwards");

    epoch.as_millis()
}

pub fn _round_to_place(num: f64, place: u32) -> f64 {
    let place = 10.0_f64.powi(place as i32);
    (num * place).round() / place
}