use lrncore::logs;

use crate::nxfs::{self, config::LogLevel};

pub fn log_from_log_level(log_level: LogLevel, log_msg: &str) {
    let config = nxfs::config::parse_config_file();
    let config_log_level = config.unwrap().behavior.log_level;

    if log_level <= config_log_level {
        match log_level {
            LogLevel::Error => logs::time_error_log(log_msg),
            LogLevel::Warn => logs::warning_log(log_msg),
            LogLevel::Info => logs::info_log(log_msg),
            LogLevel::Debug => todo!(),
            LogLevel::Trace => todo!(),
        }
    }
}
