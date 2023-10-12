use reqwest::Client;
use serde::{Deserialize, Serialize};

use crate::prelude::*;
pub async fn get_key() -> String {
    let client: Client = Client::new();
    let res = client
        .get(SKOLA24_BASE_URL.to_string() + "/get/timetable/render/key")
        .header("X-Scope", SKOLA24_KEY)
        .send()
        .await;

    let body = res.unwrap().text().await.unwrap();
    let body_parsed: KeyResponse = serde_json::from_str(body.as_str()).expect("Har ingen key");

    body_parsed.data.key
}
#[derive(Serialize, Deserialize, Debug)]
struct KeyData {
    key: String,
}
#[derive(Serialize, Deserialize, Debug)]
struct KeyResponse {
    data: KeyData,
}
