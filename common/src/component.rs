use chrono::{DateTime, Local};
use uuid::Uuid;

use crate::{r#mod::Change, traits::Diff};

// Common fields shared across all components
#[derive(Serialize, Deserialize)]
pub struct ComponentMeta {
    pub id: Uuid,
    pub created: DateTime<Local>,
    pub edited: DateTime<Local>, // Last time this component had a mod applied to it. For mods this
    // will always be the same as created.
    pub mods: Vec<Uuid>, // List of mod uuids applied to this component. Must be empty for mods.
    pub synced: Option<DateTime<Local>>, // Last time this component was synced with the server.
                         // None if it has never been synced.
}

impl Diff for ComponentMeta {
    fn diff<T>(_old: Self, _new: Self) -> Vec<Change<T>> {
        return Vec::new();
    }
}

impl ComponentMeta {
    pub fn new() -> ComponentMeta {
        ComponentMeta {
            id: Uuid::new_v4(),
            created: Local::now(),
            edited: Local::now(),
            synced: None,
            mods: Vec::new(),
        }
    }
}
