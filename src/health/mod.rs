use std::process::Command;

use colored::Colorize;

use crate::logs;

pub fn dev_env_health() {
    logs::info_log("Dev environment health status".to_string());
    logs::info_log("Dev services: ".to_string());
    check_docker();
    logs::info_log("Dev tools: ".to_string());
    check_git();
}

fn check_docker() {
    let docker_stats = Command::new("docker")
        .arg("stats")
        .arg("--no-stream")
        .output()
        .expect("Failed to call the docker stats command");
    if docker_stats.status.code() == Some(0) {
        logs::active_log("Docker:");
    } else {
        logs::inactive_log("Docker:");
    }
}

fn check_git() {
    let git_installed = Command::new("git")
        .output()
        .expect("Failed to call the git command");
    if git_installed.status.code() == Some(0) {
        not_installed("git");
    } else {
        installed("git");
    }
}

fn not_installed(msg: &str) {
    let not_installed = "not installed".truecolor(255, 0, 0);
    println!("{} {}", msg, not_installed);
}

fn installed(msg: &str) {
    let installed = "installed".truecolor(0, 255, 0);
    println!("{} {}", msg, installed);
}
