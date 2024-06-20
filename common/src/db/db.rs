pub struct Db {
}

impl Db {
    pub fn connect() -> Result<sqlite::Connection, sqlite::Error> {
        // TODO: move to configurable file
        sqlite::open(":memory:")
    }
}
