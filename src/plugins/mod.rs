use std::{env, fs};

use lrncore::usage_exit::command_usage;

use crate::{nxfs::config::{parse_config_file, LogLevel}, utils::log::log_from_log_level};

fn plugins_help() -> &'static str {
    (r"
Usage: nyx plugins [subcommand] [arguments] [options]

Subcommands:
    check         Check plugins health

Options:
    -h, --help      Show this help message
     ") as _
}

pub fn plugins_command() {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 2 {
        command_usage(plugins_help());
    }
    match args[2].as_str() {
        "health" => health_plugins(),
        _ => {
            command_usage(plugins_help());
        }
    }
}

// Check plugins installed
fn health_plugins() {
    // Load config file
    let config = parse_config_file().expect("Failed to parse config file");
    let mut invalid_plugins_number: u8 = 0;
    // Iterate over plugins list
    for each in config.plugins.list {
        let file_path: &str = &format!("plugins/{each}.toml");
        if !fs::exists(file_path).expect("Failed to check path") {
            invalid_plugins_number += 1;
            log_from_log_level(LogLevel::Warn, format!("Failed to load plugin: {each}").as_str());
        }
    }
    // Log depending on number of invalid plugins
    if invalid_plugins_number != 0 {
        log_from_log_level(LogLevel::Warn, format!("Plugin unabled to be load: {invalid_plugins_number}").as_str());
    } else {
        log_from_log_level(LogLevel::Info, "Successfully load all plugins");
    }
}
