use reqwest::Client;
use skola24_http::{SKOLA24_BASE_URL, SKOLA24_KEY};

use crate::LessonInfo;
use crate::TimeTableResponse;

use super::ClassId;
use super::School;

use super::SchoolId;

impl School<SchoolId, ClassId> {
    pub async fn get_schema(self) -> Option<Vec<LessonInfo>> {
        let body = &serde_json::json!({
            "renderKey": skola24_http::key::get_key().await,
            "host": "it-gymnasiet.skola24.se",
            "unitGuid": self.school_id.0,
            "scheduleDay": 0,
            "width": 400,
            "height": 400,
            "selection": self.class_id.0,
            "week": 40,
            "year": 2023
        });

        let client = Client::new();
        let res = client
            .post(SKOLA24_BASE_URL.to_string() + "/render/timetable")
            .header("X-Scope", SKOLA24_KEY)
            .header("Content-Type", "application/json")
            .body(serde_json::to_string(body).unwrap())
            .send()
            .await;

        let body = res.unwrap().text().await.unwrap();

        // // Jag fixade sort lessons funktionen
        let body_parsed: TimeTableResponse = serde_json::from_str(body.as_str()).unwrap();
        body_parsed.data.lessonInfo
    }
}
