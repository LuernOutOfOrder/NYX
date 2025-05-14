use std::{
    env,
    process::{exit, Command, Stdio},
    thread::{self, JoinHandle},
};

use lrncore::usage_exit::command_usage;

use crate::{
    logs::nyx_log,
    nxfs::{blacklist, config::{parse_config_file, LogLevel}},
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
    let safe_mode = config.security.secure_mode;
    if safe_mode {
        nyx_log("Secure mode enabled. You cannot execute this operation.");
        exit(11);
    } else {
        log_from_log_level(LogLevel::Warn, "Secure mode disabled.");
        log_from_log_level(LogLevel::Warn, "Caution: You are about to execute a command from your configuration file. Make sure the command is safe and does not include potentially harmful operations.");
    }
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
    let blacklist = blacklist::read_whitelist();
    for each in blacklist {
        if cmd == each {
            log_from_log_level(LogLevel::Error, &format!("Command in the blacklist. You cannot execute this command: {:?}", cmd));
        }
    }
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
