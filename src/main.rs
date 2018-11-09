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

mod logging;

#[allow(dead_code)]
mod db;

use clap::App;
use db::datamgr;
use logging::logging_init;

fn main() {
    logging_init();
    let _matches = App::new("Garden Master")
        .version("0.1.0")
        .author("Lucas Brendel")
        .about("Help keep track of all the things that need to happen in a garden or orchard")
        .get_matches();

    let _mgr = datamgr::DataMgr::new(String::from("./data/green-thumb.db"));
}
