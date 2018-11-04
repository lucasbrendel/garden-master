use plant::{Plant, PlantType};
use rusqlite::types::{ToSql, ToSqlOutput};
use rusqlite::{Connection, Result, NO_PARAMS};
// use serde_rusqlite::*;

use std::vec::Vec;

pub struct DataMgr {
    pub conn: Connection,
}

impl ToSql for PlantType {
    fn to_sql(&self) -> Result<ToSqlOutput> {
        let ty = self.to_string();
        Ok(ToSqlOutput::from(ty))
    }
}

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
            notes TEXT,
            zones BLOB,
            plant_type TEXT NOT NULL
            )",
                NO_PARAMS,
            )
            .unwrap();
        data
    }

    pub fn save_plants(&self, plant: Plant) {
        self.conn
            .execute(
                "INSERT INTO plants (name, days_to_maturity, notes, zones, plant_type)
            VALUES (?1, ?2, ?3, ?4, ?5)",
                &[
                    &plant.name as &ToSql,
                    &plant.days_to_maturity,
                    &plant.notes,
                    &plant.zones,
                    &plant.plant_type,
                ],
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
