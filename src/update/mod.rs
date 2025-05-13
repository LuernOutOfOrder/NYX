use std::{
    env,
    process::{exit, Command, Stdio},
    thread::{self, JoinHandle},
};

use lrncore::usage_exit::command_usage;

use crate::{
    nxfs::config::{parse_config_file, LogLevel},
    utils::log::log_from_log_level,
};

fn update_help() -> String {
    let usage = r"
Usage: nyx update [subcommand] [arguments] [options]

Subcommands:
    update       Update the command specified in configuration file

Options:
    -h, --help      Show this help message

        ";
    usage.to_string()
}

pub fn update_command() {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 2 {
        update_all_commands();
        exit(0);
    }
    match args[2].as_str() {
        "" => (),

        _ => {
            command_usage(&update_help());
        }
    }
}

fn update_all_commands() {
    let config = parse_config_file().expect("Failed to parse config file");
    log_from_log_level(LogLevel::Info, "Starting updating all specified command.");
    // Contains all spawned threads
    let mut handlers: Vec<JoinHandle<()>> = Vec::new();
    // Spawn a thread per command to update
    for each in config.user.update_list {
        let thread = thread::Builder::new()
            .name(each.command.to_string())
            .spawn(move || {
                execute_update_command(&each.command, &each.sub_command);
            })
            .expect("Failed to create thread");
        handlers.push(thread);
    }
    // Join all threads
    for handle in handlers {
        handle.join().expect("Failed to join the thread");
    }
    log_from_log_level(
        LogLevel::Info,
        "Successfully updated all specified command!",
    );
}

fn execute_update_command(cmd: &str, subcmd: &str) {
    let mut command = Command::new(cmd)
        .arg(subcmd)
        .stdout(Stdio::null())
        .spawn()
        .expect(&format!("Failed to execute the command: {}", cmd));
    let wait_command = command.wait().expect("Failed to wait the command");
    if !wait_command.success() {
        log_from_log_level(LogLevel::Error, "Failed to execute the command");
    } else {
        log_from_log_level(LogLevel::Info, &format!("Successfully updated {}", cmd));
    }
}
