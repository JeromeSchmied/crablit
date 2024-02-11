extern crate output_vt100;
use crablit::config::Config;
use std::process;

fn main() {
    output_vt100::init();

    let conf = Config::fix_from_file().unwrap_or_else(|err| {
        eprintln!("Problem during parsing file: {}", err);
        process::exit(1);
    });

    if !conf.only_check() {
        if let Err(e) = crablit::run(&conf) {
            eprintln!("App error: {}", e);
            process::exit(2);
        };
    }
}
