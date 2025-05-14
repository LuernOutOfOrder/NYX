use std::{fs::File, io::Read};

use lrncore::path::change_work_dir;

use crate::utils;

pub fn read_whitelist() -> Vec<String> {
    change_work_dir(&utils::env::get_nyx_env_var());
    let mut file = File::open(".nxfs/blacklist").expect("Failed to open blacklist file");
    let mut buff: String = String::new();
    file.read_to_string(&mut buff);
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
