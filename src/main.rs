extern crate chrono;
extern crate clap;
extern crate rusqlite;

use clap::App;

mod crop;
mod plant;

use plant::{Plant, PlantType};
use rusqlite::types::ToSql;
use rusqlite::{Connection, NO_PARAMS};

fn main() {
    let _matches = App::new("Garden Master")
        .version("0.1.0")
        .author("Lucas Brendel")
        .about("Help keep track of all the things that need to happen in a garden or orchard")
        .get_matches();

    let tomato = Plant::new(String::from("Tomato"), 50, PlantType::Annual);

    let conn = Connection::open_in_memory().unwrap();

    conn.execute(
        "CREATE TABLE plants (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            days_to_maturity INTEGER
            )",
        NO_PARAMS,
    )
    .unwrap();
    conn.execute(
        "INSERT INTO plants (name, days_to_maturity)
            VALUES (?1, ?2)",
        &[&tomato.name as &ToSql, &tomato.days_to_maturity],
    )
    .unwrap();

    let mut stmt = conn
        .prepare("SELECT id, name, days_to_maturity FROM plants")
        .unwrap();
    let plants = stmt
        .query_map(NO_PARAMS, |row| {
            Plant::new(row.get(1), row.get(2), PlantType::Annual)
        })
        .unwrap();

    for pl in plants {
        println!("Found plant {:?}", pl.unwrap());
    }
}
