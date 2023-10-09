pub struct School {
    school_id: Option<String>,
}
// TOdo
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Lesson {
    id: String,
    texts: Vec<String>,
    timeStart: String,
    timeEnd: String,
    dayOfWeekNumber: i8,
    blockName: String,
}

use std::collections::HashMap;

use reqwest::Client;
use serde::{Deserialize, Serialize};

use crate::KeyResponse;

#[derive(Serialize, Deserialize, Hash, Eq, PartialEq, Debug, Clone)]
pub enum Day {
    Måndag,
    Tisdag,
    Onsdag,
    Torsdag,
    Fredag,
}
const SKOLA24_KEY: &str = "8a22163c-8662-4535-9050-bc5e1923df48";
const SKOLA24_BASE_URL: &str = "https://web.skola24.se/api";

impl School {
    async fn get_key() -> String {
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
    pub async fn new() -> Self {
        Self { school_id: None }
    }
    pub fn select_school(mut self, school_id: impl Into<String>) -> School {
        self.school_id = Some(school_id.into());
        self
    }
    pub async fn get_day_schema(
        &self,
        klass_id: impl Into<String>,
        day: Day,
    ) -> Option<Vec<Lesson>> {
        let mut body_to_send = HashMap::new();

        body_to_send.insert("renderKey", JsonValue::Text(Self::get_key().await));
        body_to_send.insert(
            "host",
            JsonValue::Text("it-gymnasiet.skola24.se".to_string()),
        );
        body_to_send.insert(
            "unitGuid",
            JsonValue::Text(self.school_id.clone().expect("No school selected")),
        );
        body_to_send.insert("scheduleDay", JsonValue::Nummer(0));
        body_to_send.insert("width", JsonValue::Nummer(400));
        body_to_send.insert("height", JsonValue::Nummer(400));
        body_to_send.insert("selection", JsonValue::Text(klass_id.into()));
        body_to_send.insert("week", JsonValue::Nummer(40));
        body_to_send.insert("year", JsonValue::Nummer(2023));

        let client = Client::new();
        let res = client
            .post(SKOLA24_BASE_URL.to_string() + "/render/timetable")
            .header("X-Scope", SKOLA24_KEY)
            .header("Content-Type", "application/json")
            .body(serde_json::to_string(&body_to_send).unwrap())
            .send()
            .await;

        let body = res.unwrap().text().await.unwrap();

        println!("{}", body);

        // // Jag fixade sort lessons funktionen
        let body_parsed: TimeTableResponse = serde_json::from_str(body.as_str()).unwrap();
        body_parsed.data.lessonInfo
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
enum JsonValue {
    Text(String),
    Nummer(i32),
}

#[derive(Serialize, Deserialize, Debug)]
struct TimeTableData {
    lessonInfo: Option<Vec<Lesson>>,
}

#[derive(Serialize, Deserialize, Debug)]
struct TimeTableResponse {
    data: TimeTableData,
    error: Option<String>,
}
