use std::process::exit;
use chrono::Local;
use std::fs::OpenOptions;
use std::io::Write;

use lrncore::logs;

use crate::nxfs::{self, config::LogLevel};

pub fn log_from_log_level(log_level: LogLevel, log_msg: &str) {
    let config = nxfs::config::parse_config_file();
    let config_log_level = config.clone().unwrap().behavior.log_level;
    let save_file = config.clone().unwrap().behavior.save_logs;

    if save_file {
        let timestamp = Local::now();
        let log = format!("{} {}", timestamp, log_msg); 
        println!("debug: {:?}", log);
        let file_path = config.unwrap().internal_path.logs;
        let mut file = OpenOptions::new()
            .write(true)
            .append(true)
            .open(file_path)
            .unwrap();

        if let Err(e) = writeln!(file, "A new line!") {
            eprintln!("Couldn't write to file: {}", e);
        }
    }

    if log_level <= config_log_level {
        match log_level {
            LogLevel::Error => {
                logs::time_error_log(log_msg);
                exit(1)
            }
            LogLevel::Warn => logs::warning_log(log_msg),
            LogLevel::Info => logs::info_log(log_msg),
            LogLevel::Debug => todo!(),
            LogLevel::Trace => todo!(),
        }
    }
}
