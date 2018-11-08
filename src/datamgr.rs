use rusqlite::{Connection, NO_PARAMS};

pub struct DataMgr {
    pub conn: Connection,
}

impl DataMgr {
    pub fn new(db: String) -> Self {
        let data = DataMgr {
            conn: Connection::open(db).unwrap(),
        };
        data.conn
            .execute(
                "CREATE TABLE IF NOT EXISTS plants (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            days_to_maturity INTEGER,
            notes TEXT,
            zones BLOB,
            plant_type TEXT NOT NULL
            )",
                NO_PARAMS,
            ).unwrap();
        data
    }
}
