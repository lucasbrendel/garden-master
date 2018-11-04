use simplelog::*;
use std::fs::File;

pub fn logging_init() {
    CombinedLogger::init(
        vec![
            TermLogger::new(LevelFilter::Warn, Config::default()).unwrap(),
            WriteLogger::new(LevelFilter::Info, Config::default(), File::create("green-thumb.log").unwrap()),
        ]
    ).unwrap();
}