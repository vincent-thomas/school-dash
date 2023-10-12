use serde::{Deserialize, Serialize};
// BORT SÅ FORT SOM MÖJLIGT
#[derive(Serialize, Deserialize, Debug)]
pub struct ClassesResponse {
    pub data: ClassesData,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct ClassesData {
    pub classes: Vec<Class>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Class {
    pub groupGuid: String,
    pub groupName: String,
    pub isClass: bool,
}
