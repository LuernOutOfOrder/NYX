use regex::Regex;
use std::{env, fs};

use crate::{
    projects::{self, nxp::NXPContent, Data, Project},
    utils,
};

use super::nxs;

pub fn project_update_help() -> String {
    let usage = r"
Usage: nyx project-update

Options:

    -h, --help      Show this help message
";

    return usage.to_string();
}

pub fn update_project_properties() {
    let args: Vec<String> = env::args().collect();
    if let Some(arg) = args.iter().last() {
        match arg.as_str().trim() {
            "-h" => {
                utils::command_usage(&project_update_help());
            }
            "--help" => {
                utils::command_usage(&project_update_help());
            }
            _ => {}
        }
    }
    let app_data_path = utils::get_app_data();
    let mut projects = nxs::get_all_project_entries();
    inquire::set_global_render_config(utils::get_render_config());
    let app_name = utils::prompt_message(
        "Enter project id".to_string(),
        "Error with the project id referred".to_string(),
    );
    // if an index match the given data,
    let property = utils::get_select_option(
        "Which property do you want to update ?".to_string(),
        utils::get_project_property(),
    )
    .expect("Failed to get select option");
    let current_selected_project: NXPContent;
    if let Some(pos) = projects.iter().position(|app| app.name == app_name) {
        println!("Project found");
        let app = projects.remove(pos);
        println!("check output: {:?}", app);
        current_selected_project = NXPContent {
            name: app.name,
            tech: app.tech,
            location: app.location,
            repository: app.repository,
            github_project: app.github_project,
            version: app.version,
            todo: app.todo,
        };
        // let updated_project = update_select_properties(current_selected_project, property);
        // projects.push(updated_project);
        // let update_data = Data { project: projects };
        // let save_json = serde_json::to_string(&update_data).expect("Failed to serialize data");
        // fs::write(app_data_path, save_json).expect("Failed to write updated data");
    }
}
