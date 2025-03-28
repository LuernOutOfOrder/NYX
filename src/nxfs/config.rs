use std::{fs::File, io::Write, process::exit};

use lrncore::path::change_work_dir;
use lrncore::usage_exit::command_usage;
use std::env;
use crate::utils;
use std::fmt::Debug;
use serde::Deserialize;

fn config_help() -> String {
    let usage = r"
Usage: nyx config [subcommand] [arguments] [options]

Subcommands:
    init         Initialize the config file


Options:
    -h, --help      Show this help message

        ";
    usage.to_string()
}
#[derive(Debug, Deserialize)]
struct Config {
    config: ConfigHeader,
    git: ConfigGit
}

#[derive(Debug, Deserialize)]
struct ConfigHeader {
    format: String,
    version: String,
}

#[derive(Debug, Deserialize)]
struct ConfigGit {
    profile_url: String,
}

fn config_template() -> String {
    let template = r"[config]
format = 'nxs_config'
version = '0.1.0'

[git]
profile_url = ''

        ";
    template.to_string()
}

pub fn config_command() {
    change_work_dir(&utils::env::get_nyx_env_var());
    let args: Vec<String> = env::args().collect();
    if args.len() <= 2 {
        command_usage(&config_help());
    }
    match args[2].as_str() {
        "init" => init_config(),
        
        _ => {
            lrncore::logs::warning_log("Unknown command");
            command_usage(&config_help());
        }
    }
}

fn init_config() {
    let config_path = ".nxfs/config.toml".to_string();
    let mut config_file = match File::create_new(config_path) {
        Ok(f)=> f,
        Err(e) => {
            lrncore::logs::error_log(&format!("Failed to initialize config file: {}", e));
            return;      
        }
    };
    let template = config_template();
    let buf = template.as_bytes();
    match config_file.write_all(buf) {
        Ok(_) => (),
        Err(e) => {
            lrncore::logs::error_log(&format!("Failed to write the config file: {}", e));
            exit(1);      
        }
    };
    lrncore::logs::info_log("Successfully initialized nyx config file!");
    parse_config_file();
}

fn parse_config_file() {
    let config_path = ".nxfs/config.toml".to_string();
    let file = std::fs::read_to_string(config_path).expect("Failed to read the config file to string");
    let config_config: Config = match toml::from_str(&file) {
        Ok(c) => c,            
        Err(e) => {
            lrncore::logs::error_log(&format!("Failed to write the config file: {}", e));
            exit(1);      
        }
    };

    println!("debug {:?}", config_config);
}
