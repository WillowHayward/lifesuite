#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

pub mod log;
pub mod person;
pub mod place;
pub mod tag;
pub mod io;
pub mod search;

pub mod traits {
    pub mod named;
}

