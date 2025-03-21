use std::env;
use std::process::exit;

use super::nxs;
use crate::projects::nxp;
use crate::projects::nxs::{NXSHeader, ProjectEntry, ProjectList, NXS};
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
        "Enter project name:".to_string(),
        "Error with the project name referred".to_string(),
    );

    let mut current_project: ProjectEntry = ProjectEntry {
        project_name: String::new(),
        project_hash: [0u8; 11],
        project_size: 0,
    };
    if let Some(pos) = projects.iter().position(|app| app.project_name == app_name) {
        println!("Project found");
        let app = projects.remove(pos);
        current_project.project_hash = app.project_hash.clone();
        current_project.project_name = app.project_name.clone();
        current_project.project_size = app.project_size.clone();
    } else {
        lrncore::logs::error_log("Project not found");
        exit(1);
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
    let hash = String::from_utf8_lossy(&current_project.project_hash);
    nxp::parse_nxp_file(&format!(".nxfs/projects/{}", &hash), &mut nxp);
    let project_content: NXPContent = nxp.content;
    let buffer = utils::update_editor(project_content);
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
    nxp::update_nxp(&hash, buffer);
}
