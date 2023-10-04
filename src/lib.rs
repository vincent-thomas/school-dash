#![allow(non_snake_case)]

use std::collections::HashMap;
use std::fmt;

use reqwest::Client;
use serde::{Deserialize, Serialize};

const SKOLA24_KEY: &str = "8a22163c-8662-4535-9050-bc5e1923df48";
const SKOLA24_BASE_URL: &str = "https://web.skola24.se/api";

#[derive(Serialize, Deserialize, Debug)]
struct KeyData {
    key: String,
}
#[derive(Serialize, Deserialize, Debug)]
struct KeyResponse {
    data: KeyData,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LessonInfo {
    guidId: String,
    texts: Vec<String>,
    timeStart: String,
    timeEnd: String,
    dayOfWeekNumber: i8,
    blockName: String,
}

#[derive(Clone)]
pub struct Lessons {
    pub lessons: Vec<LessonInfo>,
}

impl fmt::Display for LessonInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        serde_json::to_string(self).unwrap().fmt(f)
    }
}

impl fmt::Display for Lessons {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut string = "[".to_string();
        self.lessons
            .iter()
            .for_each(|f| string.push_str(format!("{},", f).as_str()));
        string.push_str("]");
        write!(f, "{}", string)
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct TimeTableData {
    lessonInfo: Option<Vec<LessonInfo>>,
}

#[derive(Serialize, Deserialize, Debug)]
struct TimeTableResponse {
    data: TimeTableData,
    error: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
enum JsonValue {
    Text(String),
    Nummer(i32),
}

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

pub async fn get_lesson_info(client: Client, key: String) -> Lessons {
    let mut body_to_send = HashMap::new();

    body_to_send.insert("renderKey", JsonValue::Text(key));
    body_to_send.insert(
        "host",
        JsonValue::Text("it-gymnasiet.skola24.se".to_string()),
    );
    body_to_send.insert(
        "unitGuid",
        JsonValue::Text("MzMzODU1NjAtZGYyZS1mM2U2LTgzY2MtNDA0NGFjMmZjZjUw".to_string()),
    );
    body_to_send.insert("scheduleDay", JsonValue::Nummer(0));
    body_to_send.insert("width", JsonValue::Nummer(400));
    body_to_send.insert("height", JsonValue::Nummer(400));
    body_to_send.insert(
        "selection",
        JsonValue::Text("Y2MwYzVmYjktZjlkNy1mOWIzLThlN2MtMDNmNzIyNjVkNzJl".to_string()),
    );
    body_to_send.insert("week", JsonValue::Nummer(40));
    body_to_send.insert("year", JsonValue::Nummer(2023));

    let res = client
        .post(SKOLA24_BASE_URL.to_string() + "/render/timetable")
        .header("X-Scope", SKOLA24_KEY)
        .header("Content-Type", "application/json")
        .body(serde_json::to_string(&body_to_send).unwrap())
        .send()
        .await;

    let body = res.unwrap().text().await.unwrap();

    // Jag fixade sort lessons funktionen
    let body_parsed: TimeTableResponse = serde_json::from_str(body.as_str()).unwrap();

    Lessons {
        lessons: body_parsed.data.lessonInfo.unwrap(),
    }
}

#[derive(Serialize, Deserialize, Hash, Eq, PartialEq, Debug, Clone)]
pub enum Day {
    Måndag,
    Tisdag,
    Onsdag,
    Torsdag,
    Fredag,
}

pub fn serialize_day(day: i8) -> Day {
    match day {
        1 => Day::Måndag,
        2 => Day::Tisdag,
        3 => Day::Onsdag,
        4 => Day::Torsdag,
        5 => Day::Fredag,
        _ => panic!("Dag är inte mellan 1 och 5"),
    }
}

#[derive(Serialize, Deserialize, Hash, Eq, PartialEq, Debug, Clone)]
pub struct Lesson {
    id: String,
    lesson_name: String,
    teacher: Option<String>,
    start_time: String,
    end_time: String,
    day: Day,
}

/// Whaaat the fuuuuuck, så fixad
pub fn parse_lessons(lessons: Lessons) -> HashMap<Day, Vec<Lesson>> {
    // let mut sorterad = [vec![], vec![], vec![], vec![], vec![]];
    let mut sorted: HashMap<Day, Vec<LessonInfo>> = HashMap::from_iter(
        [
            Day::Måndag,
            Day::Tisdag,
            Day::Onsdag,
            Day::Torsdag,
            Day::Fredag,
        ]
        .map(|x| (x, vec![])),
    );
    lessons.lessons.iter().for_each(|f| {
        sorted
            .get_mut(&serialize_day(f.dayOfWeekNumber))
            .unwrap()
            .push(f.clone());
    });

    let mut mer_sorterad: HashMap<Day, Vec<Lesson>> = HashMap::from_iter(
        [
            Day::Måndag,
            Day::Tisdag,
            Day::Onsdag,
            Day::Torsdag,
            Day::Fredag,
        ]
        .map(|x| (x, vec![])),
    );

    for (day, lessons) in sorted.clone() {
        for lesson in lessons {
            mer_sorterad.get_mut(&day).unwrap().push(Lesson {
                id: lesson.guidId,
                lesson_name: lesson.texts[0].clone(),
                start_time: lesson.timeStart,
                end_time: lesson.timeEnd,
                teacher: lesson.texts.get(1).cloned(),
                day: day.clone(),
            })
        }
    }

    mer_sorterad
}
