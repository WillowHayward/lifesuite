use std::collections::HashSet;

use chrono::{DateTime, Local};
use sqlite::Connection;
use uuid::Uuid;

use crate::{export::ExportFormat, r#mod::Change, traits::{Diff, LocalDbTable}};

// TODO: Check how this serialization/deserialization works - string vs num
#[derive(Serialize, Deserialize)]
pub enum ComponentType {
    // Common
    Persona,
    Tag,
    Mod,

    // Journal
    Journal,
    Log,
    Template,
    Event,

    // Finance
    Ledger,
    Account,
    Transaction,

    // List
    Almanac,
    List,
}

impl ComponentType {
    pub fn get_id(&self) -> i32 {
        match self {
            // Common - 0-9
            ComponentType::Persona => 0,
            ComponentType::Tag => 1,
            ComponentType::Mod => 2,
            // Journal - 10-19
            ComponentType::Journal => 10,
            ComponentType::Log => 11,
            ComponentType::Template => 12,
            ComponentType::Event => 13,
            // Finance - 20-29
            ComponentType::Ledger => 20,
            ComponentType::Account => 21,
            ComponentType::Transaction => 22,
            // List - 30-39
            ComponentType::Almanac => 30,
            ComponentType::List => 31,
        }
    }
}

// Common fields shared across all components
#[derive(Serialize, Deserialize)]
pub struct ComponentMeta {
    pub id: Uuid,
    pub component_type: ComponentType,
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
    pub fn new(component_type: ComponentType) -> ComponentMeta {
        ComponentMeta {
            id: Uuid::new_v4(),
            component_type,
            created: Local::now(),
            edited: Local::now(),
            synced: None,
            mods: Vec::new(),
        }
    }

    pub fn export(&self, export_type: ExportFormat) -> String {
        match export_type {
            ExportFormat::Json => export_json(self),
            ExportFormat::Sql => export_sql(self),
        }
    }
}

impl LocalDbTable for ComponentMeta {
    fn table_name() -> &'static str {
        "component"
    }
    fn dependencies() -> HashSet<HashSet<&'static str>> {
        HashSet::new()
    }
    fn create(connection: Connection) -> Result<(), sqlite::Error> {
        connection.execute(
            "CREATE TABLE component (
            id TEXT PRIMARY KEY,
            component_type INTEGER,
            created TEXT,
            edited TEXT,
            synced TEXT
        )",
        )?;
        Ok(())
    }
}

pub fn export_json(_component: &ComponentMeta) -> String {
    todo!()
}

pub fn export_sql(_component: &ComponentMeta) -> String {
    todo!()
}
