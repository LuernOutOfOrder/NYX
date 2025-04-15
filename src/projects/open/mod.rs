use crate::projects::exit;
use crate::{nxfs, utils};
use lrncore::path::change_work_dir;
use std::process::Command;
use lrncore::usage_exit::command_usage;
use std::env;

use super::nxs;

pub fn open_help() -> String {
    let usage = r"
Usage: nyx project open [args]

Arguments:
    <project-name>  Enter project name to quickly open your editor

Options:

    -h, --help      Show this help message
    ";

    usage.to_string()
}

pub fn open_editor(project: &str) {
    change_work_dir(&utils::env::get_nyx_env_var());
    let args: Vec<String> = env::args().collect();
    if let Some(arg) = args.iter().last() {
        match arg.as_str().trim() {
            "-h" => {
                command_usage(&open_help());
            }
            "--help" => {
                command_usage(&open_help());
            }
            _ => {}
        }
    }
    let mut user_input_project = project.to_string();
    if project.is_empty() {
        let app_name = utils::prompt_message(
            "Enter project name:".to_string(),
            "Failed to get user input".to_string(),
        );
        user_input_project = app_name;
    }
    let mut projects = nxs::get_all_project_entries();
    let location: String;
    if let Some(pos) = projects
        .iter()
        .position(|app| app.name == user_input_project.as_str())
    {
        let app = projects.remove(pos);
        location = app.location;
    } else {
        lrncore::logs::error_log("Project not found");
        exit(1);
    }
    let config = nxfs::config::parse_config_file().expect("Failed to parse nyx config file");
    let editor_var = config.behavior.default_editor;
    change_work_dir(&location);
    Command::new(editor_var)
        .status()
        .expect("Something went wrong");
}
