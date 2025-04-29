use crate::nxfs;
use crate::nxfs::config::LogLevel;
use crate::utils::env::get_nyx_env_var;
use crate::utils::write;
use crate::utils::Command;
use crate::utils::File;
use crate::utils::NXPContent;
use lrncore::path::change_work_dir;
use std::io::Read;

use super::log;

/// The `update_editor` function updates a file using the specified editor and prints its
/// content.
///
/// Arguments:
///
/// * `buffer`: The `buffer` parameter in the `update_editor` function is a vector of unsigned 8-bit
///   integers (`Vec<u8>`) that represents the content to be written to a temporary file and opened in the
///   user's preferred text editor for editing.
pub fn update_editor(content: NXPContent) -> Vec<u8> {
    change_work_dir(&get_nyx_env_var());
    let editor: String = match nxfs::config::parse_config_file() {
        Ok(c) => c.behavior.default_editor,
        Err(e) => {
            log::log_from_log_level(
                LogLevel::Error,
                &format!("Failed to parse config file: {}", e),
            );
            return vec![];
        }
    };
    let file_path = ".nxfs/tmp/PROJECT_EDIT";
    match File::create(file_path) {
        Ok(f) => f,
        Err(e) => {
            log::log_from_log_level(
                LogLevel::Error,
                &format!("Failed to create temp file: {}", e),
            );
            return vec![];
        }
    };
    let json = match serde_json::to_string_pretty(&content) {
        Ok(str) => str,
        Err(e) => {
            log::log_from_log_level(
                LogLevel::Error,
                &format!("Failed to parse project content to JSON: {}", e),
            );
            return vec![];
        }
    };
    match write(file_path, json) {
        Ok(_) => (),
        Err(e) => {
            log::log_from_log_level(
                LogLevel::Error,
                &format!("Failed to write current project buffer to temp file: {}", e),
            );
            return vec![];
        }
    }
    Command::new(editor)
        .arg(file_path)
        .status()
        .expect("Something went wrong");

    let mut editable = String::new();
    match File::open(file_path)
        .expect("Could not open file")
        .read_to_string(&mut editable)
    {
        Ok(_) => (),
        Err(e) => {
            log::log_from_log_level(LogLevel::Error, &format!("Failed to open temp file: {}", e));
            return vec![];
        }
    }
    let update_content: NXPContent = match serde_json::from_str(&editable) {
        Ok(content) => content,
        Err(e) => {
            log::log_from_log_level(
                LogLevel::Error,
                &format!("Failed to write JSON str to struct: {}", e),
            );
            return vec![];
        }
    };
    let buffer: Vec<u8> =
        bincode::serialize(&update_content).expect("Failed to serialize updated content struct");
    buffer
}

pub fn open_new_editor(path: &str) {
    change_work_dir(&get_nyx_env_var());
    let editor: String = match nxfs::config::parse_config_file() {
        Ok(c) => c.behavior.default_editor,
        Err(e) => {
            log::log_from_log_level(
                LogLevel::Error,
                &format!("Failed to parse config file: {}", e),
            );
            "vim".to_owned()
        }
    };
    Command::new(editor)
        .arg(path)
        .status()
        .expect("Something went wrong");
}
