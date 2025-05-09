/*
This module provides utility functions for managing directory, file or using system's file system.
*/

use crate::nxfs::config::LogLevel;
use crate::utils::exit;
use crate::utils::Command;
use std::io::BufRead;
use std::io::BufReader;
use std::process::Stdio;

use super::log;

pub fn create_dir(path: &str) {
    let mut mkdir = Command::new("mkdir")
        .arg(path)
        .spawn()
        .expect("Failed to create directories");
    let wait_mkdir = mkdir.wait().expect("Failed to wait mkdir command");
    if !wait_mkdir.success() {
        log::log_from_log_level(LogLevel::Error, "Failed to execute mkdir command");
        exit(1)
    }
}

pub fn rm_command(path: String) {
    if path.is_empty() {
        log::log_from_log_level(LogLevel::Error, "Path is empty");
        exit(4)
    }

    let forbidden_names = ["/", ".", ".."];
    let forbidden_patterns = ['*', '?', '&', ';', '|', '`'];
    if forbidden_names.contains(&path.as_str()) {
        log::log_from_log_level(LogLevel::Error, "Input contains forbidden names.");
        exit(2)
    };

    if path.chars().any(|c| forbidden_patterns.contains(&c)) {
        log::log_from_log_level(
            LogLevel::Error,
            "Input contains forbidden patterns (*, ?, &, ;, |, `).",
        );
        exit(2)
    }

    let mut rm = Command::new("rm")
        .arg("-rf")
        .arg(path)
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to delete the directory of the project");
    let stdout = rm.stderr.take().expect("Failed to get stdout");
    let lines = BufReader::new(stdout).lines();
    for line in lines {
        if !line.as_ref().unwrap().is_empty() {
            log::log_from_log_level(
                LogLevel::Error,
                "Error in the remove command. Check if you have the right to remove directories.",
            );
            println!("{}", line.unwrap());
            exit(2);
        }
    }
    let wait_rm = rm.wait().expect("Failed to wait rm command");
    if !wait_rm.success() {
        log::log_from_log_level(LogLevel::Error, "Failed to execute rm command");
        exit(6);
    }
}
