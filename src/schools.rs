pub struct School {
    school_id: Option<String>,
    class_id: Option<String>,
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
        Self {
            school_id: None,
            class_id: None,
        }
    }
    pub fn select_school(mut self, school_id: impl Into<String>) -> School {
        self.school_id = Some(school_id.into());
        self
    }
    pub fn select_class_from_id(mut self, class_id: impl Into<String>) -> School {
        self.class_id = Some(class_id.into());
        self
    }
    pub async fn select_class_from_name(mut self, class_name: impl Into<String>) -> School {
        // Get all classes from https://web.skola24.se/api/get/selection

        let client = Client::new();

        let body = serde_json::json!({
            "hostName": "it-gymnasiet.skola24.se",
            "unitGuid": self.school_id,
            "filters": {
                "class": true
            }
        });

        let res = client
            .post(SKOLA24_BASE_URL.to_string() + "/get/timetable/selection")
            .header("X-Scope", SKOLA24_KEY)
            .json(&body)
            .send()
            .await
            .unwrap()
            .text()
            .await
            .unwrap();

        let body: ClassesResponse = serde_json::from_str(res.as_str()).unwrap();

        let name = class_name.into().clone();
        body.data
            .classes
            .iter()
            .filter(|class| class.groupName == name)
            .for_each(|class| {
                self.class_id = Some(class.groupGuid.clone());
            });

        self
    }
    pub async fn get_day_schema(
        &self,
        klass_id: impl Into<String>,
        day: Day,
    ) -> Option<Vec<Lesson>> {
        let body = &serde_json::json!({
            "renderKey": Self::get_key().await,
            "host": "it-gymnasiet.skola24.se",
            "unitGuid": self.school_id.clone().expect("No school selected"),
            "scheduleDay": 0,
            "width": 400,
            "height": 400,
            "selection": klass_id.into(),
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

#[derive(Serialize, Deserialize, Debug)]
struct ClassesResponse {
    data: ClassesData,
}

#[derive(Serialize, Deserialize, Debug)]
struct ClassesData {
    classes: Vec<Class>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Class {
    groupGuid: String,
    groupName: String,
    isClass: bool,
}
