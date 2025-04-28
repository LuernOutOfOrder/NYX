use inquire::Confirm;

use crate::nxfs::{self, config::LogLevel};

use super::log;

/// confirmation prompt for the user if ask_confirmation is enabled in config file
pub fn confirm_prompt(message: &str, help_message: &str) -> bool {
    let config = nxfs::config::parse_config_file().expect("Failed to parse NYX config file");
    if config.behavior.ask_confirmation {
        return Confirm::new(message)
            .with_default(false)
            .with_help_message(help_message)
            .prompt()
            .unwrap();
    }
    true
}

pub fn confirm_prompt_safe_mode(message: &str, help_message: &str) -> bool {
    let config = nxfs::config::parse_config_file().expect("Failed to parse NYX config file");
    if config.security.secure_mode {
        return Confirm::new(message)
            .with_default(false)
            .with_help_message(help_message)
            .prompt()
            .unwrap();
    }
    log::log_from_log_level(LogLevel::Warn, "Safe mode is disabled");
    true
}
