use chrono::Utc;
use std::fs::OpenOptions;
use std::io::Write;
use std::process::exit;

use lrncore::logs;

use crate::nxfs::{self, config::LogLevel};

pub fn log_from_log_level(log_level: LogLevel, log_msg: &str) {
    let config = nxfs::config::parse_config_file();
    let config_log_level = config.clone().unwrap().behavior.log_level;
    let save_file = config.clone().unwrap().behavior.save_logs;

    if save_file {
        let timestamp = Utc::now();
        let log = format!(
            "{} {}",
            timestamp.format("%d-%m-%Y %H:%M:%S").to_string(),
            log_msg
        );
        let log_path = config.unwrap().internal_path.logs;
        // log file path by day
        let daily_log_path = format!("{}{}", log_path, timestamp.format("%d-%m-%Y").to_string());
        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .append(true)
            .open(daily_log_path)
            .unwrap();

        if let Err(e) = writeln!(file, "{}", log) {
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
