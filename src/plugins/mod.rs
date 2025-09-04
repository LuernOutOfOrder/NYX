use std::{
    env, fs, path::PathBuf, process::{Child, Command}
};

use lrncore::{path::change_work_dir, usage_exit::command_usage};
use parser::parse_plugin_file;
use serde::Deserialize;

use crate::{
    nxfs::config::{parse_config_file, LogLevel},
    utils::log::log_from_log_level,
};

pub mod parser;

#[derive(Deserialize, Debug)]
pub struct Plugin {
    pub plugin: PluginSection,
    pub core: CoreSection,
    pub commands: CommandsSection,
}

#[derive(Deserialize, Debug)]
pub struct PluginSection {
    pub name: String,
    pub version: String,
    pub enabled: bool,
}

#[derive(Deserialize, Debug)]
pub struct CoreSection {
    pub build_command: bool,
    pub clean_command: bool,
    pub run_command: bool,
}

#[derive(Deserialize, Debug)]
pub struct CommandsSection {
    pub init_command: Vec<String>,
    pub build_command: Vec<String>,
    pub clean_command: Vec<String>,
    pub run_command: Vec<String>,
}

fn plugins_help() -> &'static str {
    (r"
Usage: nyx plugins [subcommand] [arguments] [options]

Subcommands:
    health         Check plugins health

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
            log_from_log_level(
                LogLevel::Warn,
                format!("Failed to load plugin: {each}").as_str(),
            );
        } else {
            match parse_plugin_file(&each) {
                Ok(_) => (),
                Err(e) => {
                    log_from_log_level(
                        LogLevel::Warn,
                        format!("Failed to parse plugin: {each}").as_str(),
                    );
                    eprintln!("{e}");
                }
            }
        }
    }
    // Log depending on number of invalid plugins
    if invalid_plugins_number != 0 {
        log_from_log_level(
            LogLevel::Warn,
            format!("Plugin unabled to be load: {invalid_plugins_number}").as_str(),
        );
    } else {
        log_from_log_level(LogLevel::Info, "Successfully load all plugins");
    }
}

/// Run init command from given plugin
pub fn run_init_command(plugin: Plugin, _name: &str, path: PathBuf) {
    change_work_dir(path.as_os_str().to_str().expect("Failed to cast pathbuf to os_str to str"));
    let mut init_commands_vec: Vec<String> = plugin.commands.init_command;
    let bin = init_commands_vec.remove(0);
    let mut command: Child = Command::new(bin)
        .args(init_commands_vec)
        .spawn()
        .expect("Failed to fork process");
    let wait_command = command.wait().expect("Failed to wait forked process"); 
    if !wait_command.success() {
        log_from_log_level(LogLevel::Error, "Failed to run init command");
    }
    log_from_log_level(LogLevel::Info, "Successfully run init command");
}
