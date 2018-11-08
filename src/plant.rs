//! The design behind the plant module is to act as the main container of data for any
//! type of plant that could be tracked and maintained in the garden or orchard.
//! This item should contain the necessary data so that work can be done for the use to alert
//! when work is needed to be performed.

use strum::AsStaticRef;
use std::str::FromStr;
use rusqlite::types::{ToSql, ToSqlOutput, FromSql, FromSqlResult, ValueRef, FromSqlError};
use rusqlite::{Connection, Result, NO_PARAMS};

/// Standard type to define all things to grow
#[derive(Debug)]
pub struct Plant {
    pub id: i64,
    /// Name of a plant.
    pub name: String,
    /// Seasonal type of plant
    pub plant_type: PlantType,
    /// Growing zones defined by USDA that a plant can survive in. This is limited to first 10 zones.
    pub zones: Vec<u8>,
    /// Any description or textual things to track about the plant.
    pub notes: String,
    /// Number of days from planting until germination occurs
    pub days_to_maturity: i64,
}

/// Seasonal variety types of plants
#[derive(Debug, PartialEq, Serialize, Deserialize, Display, EnumString, AsStaticStr)]
pub enum PlantType {
    /// Plant has one growing season and needs to be replanted every year
    Annual,
    /// Plant survives over multiple growing seasons.
    Perennial,
}

impl ToSql for PlantType {
    fn to_sql(&self) -> Result<ToSqlOutput> {
        let ty = self.to_string();
        Ok(ToSqlOutput::from(ty))
    }
}

impl FromSql for PlantType {
    fn column_result(value: ValueRef) -> FromSqlResult<Self> {
        match PlantType::from_str(value.as_str().unwrap()).unwrap() {
            PlantType::Annual => Ok(PlantType::Annual),
            PlantType::Perennial => Ok(PlantType::Perennial),
            _ => Err(FromSqlError::InvalidType)
        }
    }
}

impl Plant {
    /// Create a new plant type
    pub fn new(conn: &Connection, name: String, days_to_maturity: i64, plant_type: PlantType) -> Self {
        conn
            .execute(
                "INSERT INTO plants (name, days_to_maturity, plant_type)
            VALUES (?1, ?2, ?3)",
                &[
                    &name as &ToSql,
                    &days_to_maturity,
                    &plant_type,
                ],
            ).unwrap();
        
        Plant {
            id: conn.last_insert_rowid(),
            name,
            plant_type,
            notes: String::from(""),
            zones: Vec::new(),
            days_to_maturity,
        }
    }

    pub fn get_plants(conn: &Connection) -> Vec<Plant> {
        let mut plants: Vec<Plant> = Vec::new();
        let mut stmt = conn
            .prepare("SELECT id, name, days_to_maturity, zones, notes, plant_type FROM plants")
            .unwrap();
        let map_plants = stmt
            .query_map(NO_PARAMS, |row| {
                Plant {
                    id: row.get(0),
                    name: row.get(1),
                    days_to_maturity: row.get(2),
                    zones: row.get(3),
                    notes: row.get(4),
                    plant_type: row.get(5)
                }
                // Plant::new(row.get(1), row.get(2), PlantType::Annual)
            }).unwrap();
        for plant in map_plants {
            plants.push(plant.unwrap());
        }
        plants
    }

    pub fn get_plant_by_id(conn: &Connection, id: i64) -> Result<Plant> {
        let mut stmt = try!(conn.prepare("SELECT name, days_to_maturity, zones, notes, plant_type FROM plants WHERE id = :id LIMIT 1"));
        let plant = stmt.query_map(NO_PARAMS, |row| {
            Plant{
                id,
                name: row.get(0), 
                days_to_maturity: row.get(1),
                zones: row.get(2),
                notes: row.get(3),
                plant_type: row.get(4)
            }
        }).unwrap();
        Ok(plant.last().unwrap().unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn display_name() {
    //     let plant = Plant::new(String::from("Tomato"), 30, PlantType::Annual);
    //     assert_eq!("Tomato", plant.name);
    //     assert_eq!(30, plant.days_to_maturity);
    // }

    // #[test]
    // fn default_plant_type() {
    //     let plant = Plant::new(String::from("Potato"), 50, PlantType::Annual);
    //     assert_eq!(PlantType::Annual, plant.plant_type);
    // }
}
