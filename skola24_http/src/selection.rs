use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::prelude::*;
pub async fn get_selection(data: Value) -> Vec<Class> {
    let client: Client = Client::new();
    let res = client
        .get(SKOLA24_BASE_URL.to_string() + "/get/timetable/selection")
        .header("X-Scope", SKOLA24_KEY)
        .json(&data)
        .send()
        .await;

    let body = res.unwrap().text().await.unwrap();
    let body_parsed: SelectionResponse =
        serde_json::from_str(body.as_str()).expect("Har ingen selection");

    body_parsed.data.classes
}

#[derive(Serialize, Deserialize, Debug)]
struct SelectionData {
    classes: Vec<Class>,
}

#[derive(Serialize, Deserialize, Debug)]
struct SelectionResponse {
    data: SelectionData,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Class {
    #[serde(rename = "groupGuid")]
    pub group_guid: String,
    #[serde(rename = "groupName")]
    pub group_name: String,
}
