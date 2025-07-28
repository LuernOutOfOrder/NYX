use std::env;

use crate::{nxfs, utils::log::log_from_log_level};

pub fn init_command() {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 2 {
        init_nyx();
    }
}

// Basic init of NYX
fn init_nyx() {
    log_from_log_level(crate::nxfs::config::LogLevel::Info, "Initialize NYX..");
    nxfs::nxs::create_data();
}
