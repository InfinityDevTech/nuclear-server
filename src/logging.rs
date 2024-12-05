use std::{
    env, fs::File, path::Path
};

use fern::colors::{Color, ColoredLevelConfig};
use log::LevelFilter;

pub fn init_logger() -> LevelFilter {
    // Set the colors for the log levels
    let colors = ColoredLevelConfig::new()
        .error(Color::Red)
        .warn(Color::Yellow)
        .info(Color::Green)
        .debug(Color::Cyan)
        .trace(Color::Magenta);

    // Set the log level from config
    let log_level: LevelFilter = match env::var("LOG_LEVEL").unwrap().to_lowercase().as_str() {
        "error" => LevelFilter::Error,
        "warn" => LevelFilter::Warn,
        "info" => LevelFilter::Info,
        "debug" => LevelFilter::Debug,
        "trace" => LevelFilter::Trace,
        _ => LevelFilter::Info,
    };
    //let log_level = LevelFilter::Trace;

    // Make the logger object
    fern::Dispatch::new()
        .format(move |out, message, record| {
            out.finish(format_args!(
                "{} {}[{}]\x1B[0m {}",
                chrono::Local::now().format("[%b%e %H:%M:%S]"),
                format_args!("\x1B[{}m", colors.get_color(&record.level()).to_fg_str()),
                record.level(),
                message
            ))
        })
        .level(log_level)
        .chain(fern::log_file("output.log").unwrap())
        .chain(std::io::stdout())
        .apply()
        .expect("Failed to make logger.");

    log_level
}

// Moves the old log file to the log DIR
// It assigns it a timestamp and then closes the program
// This is run at CTRL-C
// ALL PATHS SHOULD EXIT REGARDLESS OF FAILURE
pub fn clean_old() {
    let log_dir = std::env::current_dir().unwrap().join("logs");

    // Check if dir exists
    if std::fs::metadata(Path::new(&log_dir)).is_err() {
        std::fs::create_dir(log_dir.clone()).unwrap();
    }

    // If it exists, move it to the logs dir
    if File::open("output.log").is_ok() {
        let new_file = log_dir.to_str().unwrap().to_owned()
            + "\\"
            + &chrono::Local::now().format("D%Y-%m-%d-T%H-%M-%S").to_string()
            + ".log";

        let res = std::fs::rename(
            Path::new(
                &(std::env::current_dir()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .to_owned()
                    + "\\"
                    + "output.log"),
            ),
            Path::new(&new_file.clone()),
        );
        if res.is_err() {
            println!("Failed to rename log file: {:?}", res.err());
        }
    }
}