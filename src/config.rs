use crate::args::TimeLapseConfig;
use crate::number_util::{parse_duration_to_ms, DURATION_REGEX};
use regex::Regex;
use std::path::Path;

pub fn create(config: TimeLapseConfig) {
    match validate_config(&config) {
        Ok(()) => (), /* no-op */
        Err(err) => {
            eprintln!("{}", err);
            std::process::exit(1)
        }
    }

    // println!("config::create, config: {:?}", config);
    let cargo_pkg_name: &str = env!("CARGO_PKG_NAME");

    match confy::store(cargo_pkg_name, None, config) {
        Ok(_) => {
            println!(
                "Stored config file here, {:?}",
                confy::get_configuration_file_path(cargo_pkg_name, None).unwrap()
            );
        }
        Err(err) => {
            eprintln!("{:?}", err);
            std::process::exit(1);
        }
    }
}

pub fn load() -> Option<TimeLapseConfig> {
    let cargo_pkg_name: &str = env!("CARGO_PKG_NAME");

    let config_path = confy::get_configuration_file_path(cargo_pkg_name, None).unwrap();
    // need to check if directory exists because if the confy::load doesn't find the file it will create a default, not return an error
    if Path::new(&config_path).exists() {
        let config: TimeLapseConfig = confy::load(cargo_pkg_name, None).unwrap();
        Some(config)
    } else {
        None
    }
}

pub fn show() -> Result<(), &'static str> {
    match load() {
        Some(config) => {
            println!("{:?}", config);
            Ok(())
        }
        None => Err("Couldn't load config file, use the create subcommand to create one"),
    }
}

pub fn validate_config(config: &TimeLapseConfig) -> Result<(), &'static str> {
    // validate duration values like 5d, 1w, 6h
    if !Regex::new(DURATION_REGEX)
        .unwrap()
        .is_match(&config.capture_duration)
    {
        return Err("Invalid value for capture_duration");
    }
    if !Regex::new(DURATION_REGEX)
        .unwrap()
        .is_match(&config.delay_start_duration)
    {
        return Err("Invalid value for delay_start_duration");
    }
    if parse_duration_to_ms(&config.capture_duration)? < 60 * 1000 {
        return Err("capture_duration must be at least 1 minute");
    }
    Ok(())
}
