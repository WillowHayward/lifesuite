use std::collections::HashSet;

use sqlite::Connection;
use uuid::Uuid;

use crate::r#mod::Change;

pub trait Diff {
    fn diff<T>(_old: Self, _new: Self) -> Vec<Change<T>>;
}

// TODO: Actually implement this on some structs
pub trait Get<T> {
    fn get_by_name(_name: &str) -> Option<T>;
    fn get_by_id(_id: &Uuid) -> Option<T>;
}


pub trait LocalDbTable {
    fn table_name() -> &'static str;
    fn dependencies() -> HashSet<HashSet<&'static str>>;
    fn create(connection: Connection) -> Result<(), sqlite::Error>;
}
