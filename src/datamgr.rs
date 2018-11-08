use plant::{Plant, PlantType};
use rusqlite::types::{ToSql, ToSqlOutput};
use rusqlite::{Connection, Result, NO_PARAMS};
// use serde_rusqlite::*;

use std::vec::Vec;

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
