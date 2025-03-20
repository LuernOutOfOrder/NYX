use std::env;

use super::nxs;
use crate::projects::nxp;
use crate::{
    projects::nxp::{NXPContent, NXPHeader, NXP},
    utils,
};

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
    let mut projects = nxs::get_all_project();
    inquire::set_global_render_config(utils::get_render_config());
    let app_name = utils::prompt_message(
        "Enter project name".to_string(),
        "Error with the project name referred".to_string(),
    );

    let current_selected_project: NXPContent;
    let mut hash: String = String::new();
    if let Some(pos) = projects.iter().position(|app| app.project_name == app_name) {
        println!("Project found");
        let app = projects.remove(pos);
        hash = String::from_utf8_lossy(&app.project_hash).to_string();
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
            todo: String::new(),
        },
    };
    nxp::parse_nxp_file(&format!(".data/projects/{}", hash), &mut nxp);
    let project_content: NXPContent = nxp.content;
    let buffer = utils::update_editor(project_content);
    println!("check output: {:?}", String::from_utf8_lossy(&buffer));
}
