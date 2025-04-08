use std::env;
use std::process::Command;
mod helper;

use helper::{installed, not_installed, warning};

use crate::logs;
use crate::nxfs;
use crate::vec_of_strings;
use lrncore::usage_exit::command_usage;

pub fn health_help() -> String {
    let usage = r"
Usage: nyx health [options]

Options:

    -h, --help      Show this help message
";

    usage.to_string()
}

pub fn dev_env_health() {
    let args: Vec<String> = env::args().collect();
    if let Some(arg) = args.iter().last() {
        match arg.as_str().trim() {
            "-h" => {
                command_usage(&health_help());
            }
            "--help" => {
                command_usage(&health_help());
            }
            _ => {}
        }
    }
    logs::info_log("Development environment health status:".to_string());
    logs::nyx_log("[System Requirements]");
    check_tech();
    logs::nyx_log("[Environments variables]");
    check_var();
    logs::nyx_log("[NYX Environment]");
    check_config_file();
    logs::nyx_log("[Optionnal Tools]");
    check_docker();
    logs::info_log("Health check done".to_string());
}

fn check_docker() {
    let docker_stats = Command::new("docker")
        .arg("stats")
        .arg("--no-stream")
        .output()
        .expect("Failed to call the docker stats command");
    if docker_stats.status.code() == Some(0) {
        logs::active_log("\tDocker");
    } else {
        warning("Docker");
    }
}

fn check_tech() {
    let tech_vec = vec_of_strings!("Git", "rustup", "go", "node");
    for tech in tech_vec {
        match Command::new(tech.clone())
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .spawn()
        {
            Ok(_) => {
                installed(&tech);
            }
            Err(_) => {
                not_installed(&tech);
            }
        }
    }
}

fn check_var() {
    let env_var_vec = vec_of_strings!("PATH", "RUSTUP_HOME");
    for var in env_var_vec {
        let env_command = Command::new("printenv")
            .arg(var.clone())
            .stderr(std::process::Stdio::null())
            .output()
            .expect("Failed to call the printenv command");

        if !env_command.stdout.is_empty() {
            installed(&var);
        } else {
            warning(&var);
        }
    }
}

fn check_config_file() {
    let config = nxfs::config::parse_config_file();
    match config {
        Ok(_) => {
            installed("Configuration file");
        }
        Err(_) => {
            not_installed("Configuration file");
        }
    }
}
