/*
This module provides utility functions for managing directory, file or using system's file system.
*/

use crate::utils::exit;
use crate::utils::Command;
use std::io::BufRead;
use std::io::BufReader;
use std::process::Stdio;

pub fn create_dir(path: &str) {
    let mut mkdir = Command::new("mkdir")
        .arg(path)
        .spawn()
        .expect("Failed to create directories");
    let wait_mkdir = mkdir.wait().expect("Failed to wait mkdir command");
    if !wait_mkdir.success() {
        lrncore::logs::error_log("Failed to execute mkdir command");
        exit(1)
    }
}

pub fn rm_command(path: String) {
    let forbidden_names = ["/", ".", ".."];
    let forbidden_patterns = ['*', '?', '&', ';', '|', '`'];
    if forbidden_names.contains(&path.as_str()) {
        lrncore::logs::time_error_log("Input contains forbidden names.");
        exit(1)
    };

    if path.chars().any(|c| forbidden_patterns.contains(&c)) {
        lrncore::logs::time_error_log("Input contains forbidden patterns (*, ?, &, ;, |, `).");
        exit(1)
    }

    if path.is_empty() {
        lrncore::logs::time_error_log("Path is empty");
        exit(1)
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
            lrncore::logs::time_error_log(
                "Error in the remove command. Check if you have the right to remove directories.",
            );
            println!("{}", line.unwrap());
            exit(1);
        }
    }
    let wait_rm = rm.wait().expect("Failed to wait rm command");
    if !wait_rm.success() {
        lrncore::logs::time_error_log("Failed to execute rm command");
        exit(1);
    }
}
