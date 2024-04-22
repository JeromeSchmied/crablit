use chrono::Local;
use crablit::{config::Config, log_path, AnyErr};
use log::*;
use std::{fs::OpenOptions, process};

fn main() -> AnyErr<()> {
    // set up logger
    fern::Dispatch::new()
        // Perform allocation-free log formatting
        .format(|out, message, record| {
            out.finish(format_args!(
                "{} [{}] {} {}",
                Local::now(),
                record.level(),
                record.target(),
                message
            ))
        })
        // Add blanket level filter -
        .level(log::LevelFilter::Info)
        // Output to stdout, files, and other Dispatch configurations
        .chain(
            OpenOptions::new()
                .create(true)
                .append(true)
                .open(log_path("crablit").expect("couldn't find log path"))?,
        )
        // Apply globally
        .apply()?;

    trace!("log level: TRACE");
    debug!("log level: DEBUG");
    info!("log level: INFO");
    warn!("log level: WARN");
    error!("log level: ERROR");

    output_vt100::init();

    let conf = Config::fix_from_file().unwrap_or_else(|err| {
        eprintln!("Problem during parsing file: {err}");
        process::exit(1);
    });

    if !conf.only_check {
        if let Err(e) = crablit::run(&conf) {
            eprintln!("App error: {e}");
            process::exit(2);
        };
    }

    Ok(())
}
