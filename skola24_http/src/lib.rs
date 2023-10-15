use serde::{Deserialize, Serialize};

pub mod key;
pub mod selection;
pub mod timetable;
pub(crate) mod prelude;
pub use prelude::SKOLA24_BASE_URL;
pub use prelude::SKOLA24_KEY;

// Den här skiten asså
// Den borde inte ligga här utan den borde flyttas till någon annan crate eller något
// den används av både skola24_http och schooldash
#[derive(Hash, Eq, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub enum Day {
    Måndag,
    Tisdag,
    Onsdag,
    Torsdag,
    Fredag,
    All
}