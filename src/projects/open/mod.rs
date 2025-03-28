use crate::projects::exit;
use crate::utils;
use lrncore::path::change_work_dir;
use std::process::Command;

use super::nxs;

pub fn open_editor(project: &str) {
    change_work_dir(&utils::env::get_nyx_env_var());
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

    let editor_var = utils::env::get_editor_env_var();
    Command::new(editor_var)
        .arg(&location)
        .status()
        .expect("Something went wrong");
}
