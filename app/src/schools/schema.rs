use super::{ClassId, School, SchoolId};
use skola24_http::timetable::Lesson;

impl School<SchoolId, ClassId> {
    pub async fn get_schema(self) -> Vec<Lesson> {
        let body = serde_json::json!({
            "host": "it-gymnasiet.skola24.se",
            "unitGuid": self.school_id.0,
            "scheduleDay": 0,
            "week": 40,
            "year": 2023
        });

        let lessons = skola24_http::timetable::get_timetable(body).await;

        lessons
    }
}
