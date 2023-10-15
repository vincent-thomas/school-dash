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
