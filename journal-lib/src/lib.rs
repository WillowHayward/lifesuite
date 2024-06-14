#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

pub mod log {
    pub mod log;
    pub use log::*;
    pub mod search;
}

pub mod tag {
    pub mod tag;
    pub use tag::*;
    pub mod entity;
    pub mod context;
}

pub mod commands;
pub mod index;

pub mod traits {
    pub mod named;
}

