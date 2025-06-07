/*
 This module provides functionality to update project properties in the NYX application.

 The main functions include:
 - `project_update_help`: Generates a help message for the `project-update` command.
 - `update_project_properties`: Updates the properties of a project based on user input.

 The module interacts with the following components:
 - `nxs`: For handling project entries and updating project lists.
 - `nxp`: For parsing and updating project files.
 - `utils`: For utility functions such as prompting messages and rendering configurations.

 # Usage

 To display the help message for the `project-update` command:
 ```sh
 nyx project-update -h
 ```
*/

use std::env;
use std::process::exit;

use super::nxs;
use crate::nxfs::config::LogLevel;
use crate::projects::nxp;
use crate::projects::nxs::{NXSHeader, ProjectEntry, ProjectList, NXS};
use crate::utils::editor::update_editor;
use crate::utils::log::log_from_log_level;
use crate::{
    projects::nxp::{NXPContent, NXPHeader, NXP},
    utils,
};
use lrncore::usage_exit::command_usage;

pub fn project_update_help() -> String {
    let usage = r"
Usage: nyx project-update

Options:

    -h, --help      Show this help message
";

    usage.to_string()
}

pub fn update_project_properties() {
    let args: Vec<String> = env::args().collect();
    if let Some(arg) = args.iter().last() {
        match arg.as_str().trim() {
            "-h" => {
                command_usage(&project_update_help());
            }
            "--help" => {
                command_usage(&project_update_help());
            }
            _ => {}
        }
    }
    let mut projects = nxs::get_all_project();
    inquire::set_global_render_config(utils::get_render_config());
    let app_name = utils::prompt_message(
        "Enter project name:",
        "Error with the project name referred",
    );

    let mut current_project: ProjectEntry = ProjectEntry {
        project_name: String::new(),
        project_hash: [0u8; 11],
        project_size: 0,
    };
    if let Some(pos) = projects.iter().position(|app| app.project_name == app_name) {
        log_from_log_level(LogLevel::Info, "Project found");
        let app = projects.remove(pos);
        current_project.project_hash = app.project_hash;
        current_project.project_name = app.project_name;
        current_project.project_size = app.project_size;
    } else {
        log_from_log_level(LogLevel::Error, "Project not found");
        exit(10);
    }
    let mut nxp: NXP = NXP {
        header: NXPHeader {
            magic_number: [0; 4],
            format_version: [0; 6],
            project_id: [0; 11],
            project_size: 0,
            reserved: 0,
        },
        content: NXPContent {
            name: String::new(),
            tech: String::new(),
            location: String::new(),
            repository: String::new(),
            github_project: String::new(),
            version: String::new(),
        },
    };
    let hash = str::from_utf8(&current_project.project_hash).unwrap();
    nxp::parse_nxp_file(&format!(".nxfs/projects/{}/content", &hash), &mut nxp);
    let project_content: NXPContent = nxp.content;
    let buffer = update_editor(project_content);
    let updated_content: NXPContent =
        bincode::deserialize(&buffer).expect("Failed to deserialize updated content buffer");
    if updated_content.name != current_project.project_name
        || current_project.project_size != buffer.len() as u32
    {
        let new_project_entry: ProjectEntry = ProjectEntry {
            project_name: updated_content.name,
            project_hash: current_project.project_hash,
            project_size: buffer.len() as u32,
        };
        projects.push(new_project_entry);
        let mut update_nxs: NXS = NXS {
            header: NXSHeader {
                magic_number: [0u8; 4],
                format_version: [0u8; 6],
                project_count: 0,
                reserved: 0,
            },
            projects: ProjectList { entries: vec![] },
        };
        nxs::update_project_entries(&mut update_nxs, projects);
    }
    nxp::update_nxp(hash, buffer);
}
