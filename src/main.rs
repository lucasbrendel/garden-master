extern crate chrono;
extern crate clap;

use clap::App;

mod crop;
mod plant;

fn main() {
    let _matches = App::new("Garden Master")
        .version("0.1.0")
        .author("Lucas Brendel")
        .about("Help keep track of all the things that need to happen in a garden or orchard")
        .get_matches();
}
