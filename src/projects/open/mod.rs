use lrncore::path::change_work_dir;
use crate::utils;
use std::process::Command;
use crate::projects::exit;

use super::nxs;

pub fn open_editor() {
    change_work_dir(&utils::env::get_nyx_env_var());
    let app_name = utils::prompt_message("Enter project name:".to_string(), "Failed to get user input".to_string());
    let mut projects = nxs::get_all_project_entries();
    let location: String;
    if let Some(pos) = projects.iter().position(|app| app.name == app_name) {
        let app = projects.remove(pos);
        location = app.location;
    } else {
        lrncore::logs::error_log("Project not found");
        exit(1);
    }

    let editor_var = utils::env::get_editor_env_var();
        Command::new(editor_var)
        .arg(&location)
        .status()
        .expect("Something went wrong");
}

