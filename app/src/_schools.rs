// TODO:
// - Fix the get_day_schema function
// - - Return data as a vector of lessons for the selected day
// - - Use the school class id instead of the parameter.

// HELA DENNA FILENS LOGIK SKA BORT. DEN ÄR GAMMAL MEN FUNKTIONELL

use skola24_http::Day;

pub struct School {
    school_id: Option<String>,
    class_id: Option<String>,
}
// TOdo
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ResponseLesson {
    pub guidId: String,
    pub texts: Vec<String>,
    pub timeStart: String,
    pub timeEnd: String,
    pub dayOfWeekNumber: i8,
    pub blockName: String,
}

#[derive(Debug, Clone)]
pub struct Lesson {
    pub id: String,
    pub lesson_name: String,
    pub teacher: Option<String>,
    pub start_time: String,
    pub end_time: String,
    pub day: Day,
}

pub struct Lessons {
    pub lessons: Vec<Lesson>,
}

use reqwest::Client;
use serde::{Deserialize, Serialize};
use skola24_http::key::get_key;

const SKOLA24_KEY: &str = "8a22163c-8662-4535-9050-bc5e1923df48";
const SKOLA24_BASE_URL: &str = "https://web.skola24.se/api";

impl School {
    pub async fn new() -> Self {
        Self {
            school_id: None,
            class_id: None,
        }
    }
    pub fn select_school(mut self, school_id: String) -> School {
        self.school_id = Some(school_id);
        println!("School id: {}", self.school_id.clone().unwrap());
        self
    }
    pub fn select_class_from_id(mut self, class_id: String) -> School {
        self.class_id = Some(class_id);
        println!("Class id: {}", self.class_id.clone().unwrap());
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
    pub async fn get_day_schema(&self, _day: Day) -> Option<Vec<Lesson>> {
        let body = &serde_json::json!({
            "renderKey": get_key().await,
            "host": "it-gymnasiet.skola24.se",
            "unitGuid": self.school_id.clone().unwrap(),
            "scheduleDay": 0,
            "width": 400,
            "height": 400,
            "selection": self.class_id.clone().unwrap(),
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

        print!("School id: {}", self.school_id.clone().unwrap());
        print!("Class id: {}", self.class_id.clone().unwrap());
        println!("{}", body);

        let _lessons = body_parsed.data.lessonInfo.unwrap().clone();

        // Turn the lessons into a vector of lessons instead of ResponseLessons

        // Bästa error-fixen
        return None;

        // Some(
        //     lessons
        //         .into_iter()
        //         // .map(response_lesson_to_lesson)
        //         // Någon error jag inte vet vad de är
        //         // .filter(|lesson| lesson.day == day)
        //         .collect::<Vec<Lesson>>(),
        // )
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
    lessonInfo: Option<Vec<ResponseLesson>>,
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
