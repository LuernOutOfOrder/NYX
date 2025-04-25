use inquire::Confirm;

use crate::nxfs;

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
