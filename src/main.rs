extern crate chrono;
extern crate clap;
extern crate rusqlite;
#[macro_use]
extern crate serde_derive;
extern crate serde_rusqlite;
extern crate strum;
#[macro_use]
extern crate strum_macros;
#[macro_use]
extern crate log;
extern crate simplelog;

mod crop;
mod datamgr;
mod logging;
mod plant;

use clap::App;
use logging::logging_init;
use plant::{Plant, PlantType};

fn main() {
    logging_init();
    let _matches = App::new("Garden Master")
        .version("0.1.0")
        .author("Lucas Brendel")
        .about("Help keep track of all the things that need to happen in a garden or orchard")
        .get_matches();

    //     let tomato = Plant::new(String::from("Tomato"), 50, PlantType::Annual);
        let mgr = datamgr::DataMgr::new(String::from("./data/green-thumb.db"));
        let plant = Plant::new(&mgr.conn, String::from("Tomato"), 45, PlantType::Annual);
        for pl in Plant::get_plants(&mgr.conn){
            info!("{:?}", pl);
        }
    //     mgr.save_plants(tomato);
    //     for pl in mgr.get_plants() {
    //         info!("Found plant {:?}", pl);
    //     }
}
