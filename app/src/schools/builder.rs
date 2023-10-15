use super::{ClassId, NoClass, NoSchool, School, SchoolId};

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
        let body = serde_json::json!({
            "hostName": "it-gymnasiet.skola24.se",
            "unitGuid": self.school_id.0,
            "filters": {
                "class": true
            }
        });

        let classes = skola24_http::selection::get_selection(body).await;

        let mut to_return = String::new();

        classes
            .iter()
            .filter(|class| class.group_name == class_name)
            .for_each(|class| {
                to_return = class.group_guid.clone();
            });

        School {
            school_id: self.school_id.clone(),
            class_id: ClassId(to_return),
        }
    }
}
