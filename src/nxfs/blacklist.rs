/*
Module to manage the blacklist file use to avoid specific command that can potentially cause harmful operations
*/
use std::{fs::File, io::Read};


use lrncore::path::change_work_dir;

use crate::utils;

/// Read and return the content of the blacklist file.
/// The blacklist file contain all command to avoid that can break the user system.
/// The function parse the content of the file, sort it and return the list as a String vector.
pub fn read_whitelist() -> Vec<String> {
    change_work_dir(&utils::env::get_nyx_env_var());
    let mut file = File::open(".nxfs/blacklist").expect("Failed to open blacklist file");
    let mut buff: String = String::new();
    file.read_to_string(&mut buff).expect("Failed to read the file content");
    let parse_buff: Vec<&str> = buff.split("\n").collect();
    let mut blacklist: Vec<String> = Vec::new();
    for each in parse_buff {
        if each.is_empty() {
            continue;
        }
        blacklist.push(each.to_owned());
    }
    blacklist
}
