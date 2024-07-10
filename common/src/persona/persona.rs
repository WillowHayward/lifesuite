use std::collections::HashSet;

use uuid::Uuid;

use crate::{
    component::{db::ComponentTable, ComponentMeta, ComponentType},
    db::Db,
    traits::LocalDbTable,
};

#[derive(Serialize, Deserialize)]
pub struct Persona {
    pub meta: ComponentMeta,
    pub name: String,
}

impl Persona {
    pub fn new(name: String) -> Persona {
        Persona {
            meta: ComponentMeta::new(ComponentType::Persona),
            name,
        }
    }

    pub fn get_by_name(_name: &str) -> Option<Persona> {
        todo!();
    }

    pub fn get_by_id(_id: Uuid) -> Option<Persona> {
        todo!();
    }

    pub fn get_current() -> Persona {
        todo!();
    }
}

impl LocalDbTable for Persona {
    fn table_name() -> &'static str {
        "persona"
    }
    fn dependencies() -> HashSet<&'static str> {
        let mut deps = HashSet::new();
        deps.insert(ComponentTable::table_name());
        deps
    }

    fn create(connection: sqlite::Connection) -> Result<(), sqlite::Error> {
        let query = "CREATE TABLE persona (
            persona_id TEXT PRIMARY KEY,
            name TEXT NOT NULL,
            FOREIGN KEY (persona_id) REFERENCES component (component_id)
        )";
        connection.execute(query)
    }
}
