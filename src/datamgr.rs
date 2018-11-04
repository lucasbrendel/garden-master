use rusqlite::types::ToSql;
use rusqlite::{Connection, NO_PARAMS};
use serde_rusqlite::*;
use plant::{Plant, PlantType};

use std::vec::Vec;

pub struct DataMgr {
    pub conn: Connection,
}

// impl ToSql for PlantType {
//     fn to_sql(&self) -> Result<ToSqlOutput> {
//         Ok(ToSqlOutput::from(self))
//     }
// }

impl DataMgr {
    pub fn new() -> Self {
        let data = DataMgr {
            conn: Connection::open("./data/green-thumb.db").unwrap(),
        };
        data.conn
            .execute(
                "CREATE TABLE IF NOT EXISTS plants (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            days_to_maturity INTEGER,
            notes TEXT
            )",
                NO_PARAMS,
            ).unwrap();
        data
    }

    pub fn save_plants(&self, plant: Plant) {
        self.conn
            .execute(
                "INSERT INTO plants (name, days_to_maturity, notes)
            VALUES (?1, ?2, ?3)",
                &[&plant.name as &ToSql, &plant.days_to_maturity, &plant.notes],
            )
            .unwrap();
    }

    pub fn get_plants(&self) -> Vec<Plant> {
        let mut plants: Vec<Plant> = Vec::new();
        let mut stmt = self
            .conn
            .prepare("SELECT id, name, days_to_maturity FROM plants")
            .unwrap();
        let map_plants = stmt
            .query_map(NO_PARAMS, |row| {
                Plant::new(row.get(1), row.get(2), PlantType::Annual)
            })
            .unwrap();
        for plant in map_plants {
            plants.push(plant.unwrap());
        }
        plants
    }
}
