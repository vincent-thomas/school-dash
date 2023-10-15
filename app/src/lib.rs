#![allow(non_snake_case)]

use std::collections::HashMap;
use std::fmt;

use _schools::Lesson;
use reqwest::Client;
use serde::{Deserialize, Serialize};

use skola24_http::Day;

pub mod _schools;
pub mod schools;

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
        string.push(']');
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

pub async fn get_lesson_info(_client: Client, key: String) -> Lessons {
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
    todo!();
}

/// Whaaat the fuuuuuck, så fixad
pub fn parse_lessons(_lessons: Lessons) -> HashMap<Day, Vec<Lesson>> {
    // let mut sorterad = [vec![], vec![], vec![], vec![], vec![]];
    let mut _sorted: HashMap<Day, Vec<LessonInfo>> = HashMap::from_iter(
        [
            Day::Måndag,
            Day::Tisdag,
            Day::Onsdag,
            Day::Torsdag,
            Day::Fredag,
        ]
        .map(|x| (x, vec![])),
    );
    // lessons.lessons.iter().for_each(|f| {
    //     sorted
    //         .get_mut(&serialize_day(f.dayOfWeekNumber))
    //         .unwrap()
    //         .push(f.clone());
    // });

    let mut _mer_sorterad: HashMap<Day, Vec<Lesson>> = HashMap::from_iter(
        [
            Day::Måndag,
            Day::Tisdag,
            Day::Onsdag,
            Day::Torsdag,
            Day::Fredag,
        ]
        .map(|x| (x, vec![])),
    );

    // for (day, lessons) in sorted.clone() {
    //     for lesson in lessons {
    //         mer_sorterad.get_mut(&day).unwrap().push(Lesson {
    //             id: lesson.guidId,
    //             lesson_name: lesson.texts[0].clone(),
    //             start_time: lesson.timeStart,
    //             end_time: lesson.timeEnd,
    //             teacher: lesson.texts.get(1).cloned(),
    //             day: day.clone(),
    //         })
    //     }
    // }

    _mer_sorterad
}

// Fin abstraktion över Self.get(&day).unwrap().clone(). Behöver inte unwrap på grund av att det är garanterat att alla nycklar från måndag till söndag finns.
pub trait LessonMap<T> {
    fn get_day(&self, day: Day) -> T;
}

type Value = Vec<Lesson>;

impl LessonMap<Value> for HashMap<Day, Value> {
    fn get_day(&self, day: Day) -> Value {
        self.get(&day)
            .expect("Alla dagar finns inte i lesson_map")
            .clone()
    }
}
