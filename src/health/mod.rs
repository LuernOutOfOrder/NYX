use std::process::Command;

use colored::Colorize;

use crate::logs;
use crate::vec_of_strings;

pub fn dev_env_health() {
    logs::info_log("Dev environment health status".to_string());
    println!("Dev services: ");
    check_docker();
    println!("\nDev tools: ");
    check_tech();
    println!("\nEnvironment var: ");
    check_var();
    logs::info_log("Health check done".to_string());
}

fn check_docker() {
    let docker_stats = Command::new("docker")
        .arg("stats")
        .arg("--no-stream")
        .output()
        .expect("Failed to call the docker stats command");
    if docker_stats.status.code() == Some(0) {
        logs::active_log("\tDocker:");
    } else {
        logs::inactive_log("\tDocker:");
    }
}

fn check_tech() {
    let tech_vec = vec_of_strings!("git", "rustup", "go", "node");
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
            println!("\t{} is present", var.clone());
        } else {
            println!("\t{} is not present", var);
        }
    }
}

fn not_installed(msg: &str) {
    let not_installed = "not installed".truecolor(255, 0, 0);
    println!("\t{} {}", msg, not_installed);
}

fn installed(msg: &str) {
    let installed = "installed".truecolor(0, 255, 0);
    println!("\t{} {}", msg, installed);
}
