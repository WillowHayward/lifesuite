use std::collections::HashSet;

use sqlite::Connection;

use crate::traits::LocalDbTable;

pub struct ComponentTable {
}

impl LocalDbTable for ComponentTable {
    fn table_name() -> &'static str {
        "component"
    }


    fn dependencies() -> HashSet<HashSet<&'static str>> {
        todo!()
    }

    fn create(connection: Connection) -> Result<(), sqlite::Error> {
        let query = "CREATE TABLE component (
            component_id TEXT PRIMARY KEY,
            created_at TEXT NOT NULL,
            synced_at TEXT
        )";
        connection.execute(query)
    }
}
