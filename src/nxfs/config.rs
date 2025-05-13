use std::process::{Command, Stdio};
use std::{fs::File, io::Write, process::exit};

use crate::utils;
use crate::utils::editor::open_new_editor;
use crate::utils::log::log_from_log_level;
use lrncore::path::change_work_dir;
use lrncore::usage_exit::command_usage;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use std::{env, fs};
use toml::de::Error as toml_error;

fn config_help() -> String {
    let usage = r"
Usage: nyx config [subcommand] [arguments] [options]

Subcommands:
    init         Initialize the config file
    update       Update the config file
    cat          Cat the config file content


Options:
    -h, --help      Show this help message

        ";
    usage.to_string()
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub config: ConfigHeader,
    pub user: ConfigUser,
    pub git: ConfigGit,
    pub behavior: ConfigBehavior,
    pub ui: ConfigUi,
    pub internal_path: ConfigInternPath,
    pub security: ConfigSecure,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ConfigHeader {
    pub format: String,
    pub version: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ConfigUser {
    pub name: String,
    pub health_list: Vec<UserHealthEntry>,
    pub update_list: Vec<UserUpdateEntry>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UserUpdateEntry {
    pub command: String,
    pub sub_command: String
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub enum UserHealthEntryCategory {
    System,
    Network,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UserHealthEntry {
    pub category: UserHealthEntryCategory,
    pub command: String,
    pub sub_command: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ConfigGit {
    pub profile_url: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ConfigBehavior {
    pub default_editor: String,
    pub auto_update: bool,
    pub ask_confirmation: bool,
    pub log_level: LogLevel,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, PartialOrd)]
pub enum LogLevel {
    Error,
    Warn,
    Info,
    Debug,
    Trace,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ConfigUi {}

#[derive(Debug, Deserialize, Serialize)]
pub struct ConfigInternPath {
    pub data: String,
    pub logs: String,
    pub cache: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ConfigSecure {
    pub secure_mode: bool,
}

fn config_template() -> String {
    let template = r"[config]
format = 'nxs_config'
version = '0.1.0'

[user]
name = ''
health_list = []
update_list = []

[git]
profile_url = ''

[behavior]
default_editor = 'vim'
auto_update = false
ask_confirmation = true
log_level = 'Info'

[ui]

[internal_path]
data = ''
logs = ''
cache = ''

[security]
secure_mode = false
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
        "update" => update_config(),
        "cat" => cat_config(),

        _ => {
            command_usage(&config_help());
        }
    }
}

fn init_config() {
    let config_path = ".nxfs/config.toml".to_string();
    let mut config_file = match File::create_new(config_path) {
        Ok(f) => f,
        Err(e) => {
            log_from_log_level(
                LogLevel::Error,
                &format!("Failed to initialize config file: {}", e),
            );
            return;
        }
    };
    let template = config_template();
    let mut config: Config = match toml::from_str(&template) {
        Ok(c) => c,
        Err(e) => {
            log_from_log_level(
                LogLevel::Error,
                &format!("Failed to deserialize config template: {}", e),
            );
            return;
        }
    };
    let ask_username = utils::prompt_message(
        "Enter a username:".to_string(),
        "Failed to get user input".to_string(),
    );
    let ask_github_profile = utils::prompt_message(
        "Enter your github profile url:".to_string(),
        "Failed to get user input".to_string(),
    );
    let data_dir = format!("{}/.nxfs/", utils::env::get_nyx_env_var());
    let log_dir = data_dir.clone() + "logs/";
    let cache_dir = data_dir.clone() + "cache/";
    config.user.name = ask_username;
    config.git.profile_url = ask_github_profile;
    config.internal_path.data = data_dir;
    config.internal_path.logs = log_dir;
    config.internal_path.cache = cache_dir;

    let config_str = toml::to_string(&config).expect("Failed to parse config struct to string");
    let buf = config_str.as_bytes();
    match config_file.write_all(buf) {
        Ok(_) => (),
        Err(e) => {
            log_from_log_level(
                LogLevel::Error,
                &format!("Failed to write the config file: {}", e),
            );
            exit(50);
        }
    };
    log_from_log_level(LogLevel::Info, "Successfully initialized nyx config file!");
}

pub fn parse_config_file() -> Result<Config, toml_error> {
    change_work_dir(&utils::env::get_nyx_env_var());
    let config_path = ".nxfs/config.toml".to_string();
    let file =
        std::fs::read_to_string(&config_path).expect("Failed to read the config file to string");
    let config: Config = match toml::from_str(&file) {
        Ok(c) => c,
        Err(e) => {
            println!("Failed to parse the configuration file: {:?}", e); 
            return Err(e);
        }
    };
    Ok(config)
}

fn update_config() {
    change_work_dir(&utils::env::get_nyx_env_var());
    let config_path = ".nxfs/config.toml";
    if !fs::exists(config_path).expect("Failed to check if the configuration path exists") {
        log_from_log_level(
            LogLevel::Error,
            "Config file path doesn't exist. Check if the configuration file exists",
        );
        exit(50);
    }
    open_new_editor(config_path);
    log_from_log_level(LogLevel::Info, "Config file updated");
}

fn cat_config() {
    change_work_dir(&utils::env::get_nyx_env_var());
    let config_path = ".nxfs/config.toml";
    if !fs::exists(config_path).expect("Failed to check if the configuration path exists") {
        log_from_log_level(
            LogLevel::Error,
            "Config file path doesn't exist. Check if the configuration file exists",
        );
        exit(50);
    }
    let cat = Command::new("cat")
        .arg(config_path)
        .stdout(Stdio::inherit())
        .output()
        .expect("Failed to execute cat command");
    println!("{}", String::from_utf8_lossy(&cat.stdout));
}
