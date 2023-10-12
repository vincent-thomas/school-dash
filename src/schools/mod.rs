use reqwest::Client;

use crate::KeyResponse;
use serde::{Deserialize, Serialize};

mod builder;
mod schema;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct School<SchoolIdPlaceholder, ClassIdPlaceholder> {
    pub school_id: SchoolIdPlaceholder,
    pub class_id: ClassIdPlaceholder,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct NoSchool;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct SchoolId(pub String);

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct NoClass;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ClassId(pub String);

impl Default for School<NoSchool, NoClass> {
    fn default() -> Self {
        Self {
            school_id: NoSchool,
            class_id: NoClass,
        }
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

const SKOLA24_KEY: &str = "8a22163c-8662-4535-9050-bc5e1923df48";
const SKOLA24_BASE_URL: &str = "https://web.skola24.se/api";

impl<T, U> School<T, U> {
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
}
