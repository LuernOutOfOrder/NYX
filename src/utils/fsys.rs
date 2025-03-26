/*
This module provide utility functions for managing directory, file or using system's file system.
*/

use crate::utils::exit;
use crate::utils::Command;

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
    Command::new("rm")
        .arg("-rf")
        .arg(path)
        .spawn()
        .expect("Failed to delete the directory of the project");
}
