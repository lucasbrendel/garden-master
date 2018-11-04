extern crate chrono;
extern crate clap;
extern crate rusqlite;
#[macro_use]
extern crate serde_derive;
extern crate serde_rusqlite;

use clap::App;

mod crop;
mod datamgr;
mod plant;

use plant::{Plant, PlantType};

fn main() {
    let _matches = App::new("Garden Master")
        .version("0.1.0")
        .author("Lucas Brendel")
        .about("Help keep track of all the things that need to happen in a garden or orchard")
        .get_matches();

    let tomato = Plant::new(String::from("Tomato"), 50, PlantType::Annual);

    let mgr = datamgr::DataMgr::new();

    mgr.save_plants(tomato);

    for pl in mgr.get_plants() {
        println!("Found plant {:?}", pl);
    }
}
