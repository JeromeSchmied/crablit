extern crate output_vt100;
use crablit::config;
use std::process;

fn main() {
    output_vt100::init();

    let config = config::Config::fix_from_file().unwrap_or_else(|err| {
        eprintln!("Problem during parsing file: {}", err);
        process::exit(1);
    });

    if !config.only_check {
        if let Err(e) = crablit::run(&config) {
            eprintln!("App error: {}", e);
            process::exit(2);
        };
    }
}
