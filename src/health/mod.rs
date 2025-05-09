use std::{env, process::exit};

use lrncore::usage_exit::command_usage;

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
    println!("user health env")
}
