#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

pub mod commands;
pub mod index;

pub mod journal {
    pub mod journal;
    pub use journal::*;
    pub mod commands;
}

pub mod log {
    pub mod log;
    pub use log::*;
    pub mod commands;
    pub mod search;
    pub mod opts;
}

pub mod tag {
    pub mod tag;
    pub use tag::*;
    pub mod commands;
    pub mod values;
    pub mod rules;

    pub mod entity {
        pub mod commands;
    }
    pub mod context {
        pub mod commands;
    }
}

pub mod template {
    pub mod template;
    pub use template::*;
    pub mod commands;
}

pub mod event {
    pub mod commands;
}

pub mod report {
    pub mod commands;
}

pub mod sync {
    pub mod commands;
}


pub mod traits {
    pub mod named;
}
