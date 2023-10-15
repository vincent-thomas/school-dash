mod builder;
mod schema;

#[derive(Clone, Debug)]
pub struct School<SchoolIdPlaceholder, ClassIdPlaceholder> {
    pub school_id: SchoolIdPlaceholder,
    pub class_id: ClassIdPlaceholder,
}

#[derive(Clone, Debug)]
pub struct NoSchool;

#[derive(Clone, Debug)]
pub struct SchoolId(pub String);

#[derive(Clone, Debug)]
pub struct NoClass;

#[derive(Clone, Debug)]
pub struct ClassId(pub String);

impl Default for School<NoSchool, NoClass> {
    fn default() -> Self {
        Self {
            school_id: NoSchool,
            class_id: NoClass,
        }
    }
}