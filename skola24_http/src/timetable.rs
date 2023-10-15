use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::key::get_key;
use crate::prelude::*;
use crate::Day;

/**
 * Get the timetable for a class
 *
 * data should contain following keys:
 * - host: String (school host)
 * - unitGuid: String (school id)
 * - scheduleDay: i8 (day of week - 0 is the entire week)
 * - selection: String (class id)
 * - week: i8
 * - year: i16
 *
 * @param data The data to send to the API
 * @returns A vector of lessons
 */
pub async fn get_timetable(mut data: Value) -> Vec<Lesson> {
    if let Some(map) = data.as_object_mut() {
        map.insert("renderKey".to_string(), serde_json::json!(get_key().await));
        map.insert("width".to_string(), serde_json::json!(400)); // Skola24 requires these
        map.insert("height".to_string(), serde_json::json!(400)); // Skola24 requires these
    }

    let client: Client = Client::new();
    let res = client
        .post(SKOLA24_BASE_URL.to_string() + "/get/timetable")
        .header("X-Scope", SKOLA24_KEY)
        .json(&data)
        .send()
        .await;

    let body = res.unwrap().text().await.unwrap();
    let body_parsed: TimeTableResponse =
        serde_json::from_str(body.as_str()).expect("Har ingen timetable");

    let response_lessons: Vec<ResponseLesson> = body_parsed.data.lesson_info.unwrap();

    let mut lessons: Vec<Lesson> = Vec::new();

    response_lessons.into_iter().for_each(|response_lesson| {
        lessons.push(Lesson {
            id: response_lesson.guidId,
            lesson_name: response_lesson.texts[0].clone(),
            teacher: if response_lesson.texts.len() > 1 {
                Some(response_lesson.texts[1].clone())
            } else {
                None
            },
            start_time: response_lesson.timeStart,
            end_time: response_lesson.timeEnd,
            day: match response_lesson.dayOfWeekNumber {
                1 => Day::Måndag,
                2 => Day::Tisdag,
                3 => Day::Onsdag,
                4 => Day::Torsdag,
                5 => Day::Fredag,
                _ => panic!("Invalid day of week"),
            },
        });
    });

    lessons
}

#[derive(Serialize, Deserialize, Debug)]
struct TimeTableData {
    lesson_info: Option<Vec<ResponseLesson>>,
}

#[derive(Serialize, Deserialize, Debug)]
struct TimeTableResponse {
    data: TimeTableData,
}

#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
struct ResponseLesson {
    guidId: String,
    texts: Vec<String>,
    timeStart: String,
    timeEnd: String,
    dayOfWeekNumber: i8,
    blockName: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Lesson {
    pub id: String,
    pub lesson_name: String,
    pub teacher: Option<String>,
    pub start_time: String,
    pub end_time: String,
    pub day: Day,
}
