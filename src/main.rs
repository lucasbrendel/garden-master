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

    let tomato = Plant::new(String::from("Tomato"), 50, 50);

    let conn = Connection::open_in_memory().unwrap();
}
