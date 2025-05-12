use std::{
    env,
    process::{exit, Command},
};

use lrncore::usage_exit::command_usage;

use crate::{
    logs,
    nxfs::{self, config::UserHealthEntryCategory},
};

pub fn health_help() -> String {
    let usage = r"
Usage: nyx hello [options]

Options:

    -h, --help      Show this help message
";

    usage.to_string()
}

pub fn health_command() {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 2 {
        user_env_health();
        exit(0);
    }
    match args[2].as_str() {
        "-h" => {
            command_usage(&health_help());
        }
        "--help" => {
            command_usage(&health_help());
        }
        _ => {
            command_usage(&health_help());
        }
    }
}

fn user_env_health() {
    let config = nxfs::config::parse_config_file().expect("Failed to get the config file");
    logs::nyx_log("User environment health status:");
    println!("[System]");
    for each in &config.user.health_list {
        if each.category == UserHealthEntryCategory::System {
            match Command::new(&each.command)
                .stdout(std::process::Stdio::null())
                .stderr(std::process::Stdio::null())
                .spawn()
            {
                Ok(_) => {
                    logs::installed(&each.command);
                }
                Err(_) => {
                    logs::not_installed(&each.command);
                }
            }
        }
    }
    println!("[Network]");
    for each in config.user.health_list {
        if each.category == UserHealthEntryCategory::Network {
            match Command::new(&each.command)
                .stdout(std::process::Stdio::null())
                .stderr(std::process::Stdio::null())
                .spawn()
            {
                Ok(_) => {
                    logs::installed(&each.command);
                }
                Err(_) => {
                    logs::not_installed(&each.command);
                }
            }
        }
    }
}
