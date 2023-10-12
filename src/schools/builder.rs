use reqwest::Client;

use crate::json_parse::ClassesResponse;

use super::{ClassId, NoClass, NoSchool, School, SchoolId};

const SKOLA24_KEY: &str = "8a22163c-8662-4535-9050-bc5e1923df48";
const SKOLA24_BASE_URL: &str = "https://web.skola24.se/api";

impl School<NoSchool, NoClass> {
    pub fn select_school(&self, school_id: &str) -> School<SchoolId, NoClass> {
        School {
            school_id: SchoolId(school_id.to_string()),
            class_id: self.class_id.clone(),
        }
    }
}

impl School<SchoolId, NoClass> {
    pub fn select_class_from_id(&self, class_id: &str) -> School<SchoolId, ClassId> {
        School {
            school_id: self.school_id.clone(),
            // TODO: Validation av class id
            class_id: ClassId(class_id.to_string()),
        }
    }
    pub async fn select_class_from_name(self, class_name: &str) -> School<SchoolId, ClassId> {
        // Get all classes from https://web.skola24.se/api/get/selection

        let client = Client::new();

        let body = serde_json::json!({
            "hostName": "it-gymnasiet.skola24.se",
            "unitGuid": self.school_id.0,
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

        let mut to_return = "".to_string();

        body.data
            .classes
            .iter()
            .filter(|class| class.groupName == class_name)
            .for_each(|class| {
                to_return = class.groupGuid.clone();
            });

        School {
            school_id: self.school_id.clone(),
            class_id: ClassId(to_return),
        }
    }
}
