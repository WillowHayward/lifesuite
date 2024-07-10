use sqlite::Connection;

pub struct Db {}

impl Db {
    pub fn connect() -> Result<sqlite::Connection, sqlite::Error> {
        // TODO: Move to EnvVar
        let db_path = "~/.life/life.db";
        sqlite::open(db_path)
    }

    pub fn table_exists(connection: Connection, table: &'static str) -> bool {
        let query = "SELECT name FROM sqlite_master WHERE type='table' AND name=?";
        let mut statement = connection.prepare(query).unwrap();
        statement.bind((1, table)).unwrap();

        match statement.next() {
            Ok(_) => true,
            Err(_) => false,
        }
    }
}
