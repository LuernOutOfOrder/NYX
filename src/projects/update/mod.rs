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
    println!("check output: {:?}", project_content);
    let project_content_buffer: Vec<u8> =
        bincode::serialize(&project_content).expect("Failed to serialize current project content");
    utils::update_editor(project_content_buffer);
}
