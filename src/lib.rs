#![allow(non_snake_case)]

// bram, har du en iter för varje dag haha
use std::fmt;
use std::collections::HashMap;

use reqwest::Client;
use serde::{Serialize, Deserialize};


/*#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}*/

#[derive(Serialize, Deserialize, Debug)]
struct KeyData {
    key: String
}
#[derive(Serialize, Deserialize, Debug)] 
struct KeyResponse {
    data: KeyData
}

#[derive(Serialize, Deserialize, Debug, Clone)] 
pub struct LessonInfo {
    guidId: String,
    texts: Vec<String>,
    timeStart: String,
    timeEnd: String,
    dayOfWeekNumber: i8,
    blockName: String
}

#[derive(Clone)]
pub struct Lessons {
    pub lessons: Vec<LessonInfo>
}

impl fmt::Display for LessonInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        serde_json::to_string(self).unwrap().fmt(f)
    }
}

impl fmt::Display for Lessons {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut string = "[".to_string();

        self.lessons.iter().for_each(|f| string.push_str(format!("{},", f).as_str()));

        string.push_str("]");

        write!(f, "{}", string)
    }
}

#[derive(Serialize, Deserialize, Debug)] 
struct TimeTableData {
    lessonInfo: Option<Vec<LessonInfo>>
}

#[derive(Serialize, Deserialize, Debug)] 
struct TimeTableResponse {
    data: TimeTableData,
    error: Option<String>
}


#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
enum JsonValue {
    Text(String),
    Nummer(i32)
}



pub async fn get_key() -> String {
    let client: Client = Client::new();
    let res = client
        .get("https://web.skola24.se/api/get/timetable/render/key")
        .header("X-Scope", "8a22163c-8662-4535-9050-bc5e1923df48")
        .send().await;

    let body = res.unwrap().text().await.unwrap();

    let body_parsed: KeyResponse = serde_json::from_str(body.as_str()).expect("Har ingen key");
    
     body_parsed.data.key
}

pub async fn get_lesson_info(client: Client, key: String) -> Lessons {
    let mut body_to_send = HashMap::new();
    
    body_to_send.insert("renderKey", JsonValue::Text(key));
    body_to_send.insert("host", JsonValue::Text("it-gymnasiet.skola24.se".to_string()));
    body_to_send.insert("unitGuid", JsonValue::Text("MzMzODU1NjAtZGYyZS1mM2U2LTgzY2MtNDA0NGFjMmZjZjUw".to_string()));
    body_to_send.insert("scheduleDay", JsonValue::Nummer(0));
    body_to_send.insert("width", JsonValue::Nummer(400));
    body_to_send.insert("height", JsonValue::Nummer(400));
    body_to_send.insert("selection", JsonValue::Text("Y2MwYzVmYjktZjlkNy1mOWIzLThlN2MtMDNmNzIyNjVkNzJl".to_string()));
    body_to_send.insert("week", JsonValue::Nummer(40));
    body_to_send.insert("year", JsonValue::Nummer(2023));

    let res = client
        .post("https://web.skola24.se/api/render/timetable")
        .header("X-Scope", "8a22163c-8662-4535-9050-bc5e1923df48")
        .header("Content-Type", "application/json")
        .body(serde_json::to_string(&body_to_send).unwrap())
        .send()
        .await;

    let body = res.unwrap().text().await.unwrap();

    // Jag fixade sort lessons funktionen
    let body_parsed: TimeTableResponse = serde_json::from_str(body.as_str()).unwrap();

    Lessons {
        lessons: body_parsed.data.lessonInfo.unwrap()
    }

}
/// Whaaat the fuuuuuck, så fixad
pub fn sort_lessons(lessons: Lessons) -> [Vec<LessonInfo>; 5] {
    let mut sorterad = [vec![], vec![], vec![], vec![], vec![]];

    lessons.lessons.iter().for_each(|f| {
        sorterad.get_mut(f.dayOfWeekNumber as usize).unwrap().push(f.clone());
    });

    sorterad
}