use crate::projects::nxp;
use crate::projects::{
    nxp::{NXPContent, NXPHeader, NXP},
    nxs::{self, NXSHeader, ProjectList, NXS},
};
use inquire::{InquireError, Select, Text};
use std::{env, process::Command};

use crate::utils;

pub fn project_delete_help() -> String {
    let usage = r"
Usage: nyx project-delete [options]

Options:

    -h, --help      Show this help message
";

    return usage.to_string();
}

pub fn select_remove_project() {
    let args: Vec<String> = env::args().collect();
    if let Some(arg) = args.iter().last() {
        match arg.as_str().trim() {
            "-h" => {
                utils::command_usage(&project_delete_help());
            }
            "--help" => {
                utils::command_usage(&project_delete_help());
            }
            _ => {}
        }
    }
    inquire::set_global_render_config(utils::get_render_config());
    let options: Vec<&str> = vec![
        "Remove project from projects list",
        "Delete completely the project",
        "Nothing",
    ];

    let ans: std::result::Result<&str, InquireError> =
        Select::new("What do you want to do ?", options).prompt();

    match ans {
        Ok(choice) => which_remove_project(choice),
        Err(_) => println!("There was an error, please try again"),
    }
}

fn which_remove_project(choice: &str) {
    match choice {
        choice if choice == "Remove project from projects list" => remove_project_from_list(),
        choice if choice == "Delete completely the project" => remove_project_from_storage(),
        _ => println!("please make a choice"),
    }
}

fn remove_project_from_list() {
    let mut projects = nxs::get_all_project();
    inquire::set_global_render_config(utils::get_render_config());
    let app_name = Text::new("Enter the name of the project:")
        .prompt()
        .expect("Failed to read project id");
    // if an index match the given data, remove it from the vector
    let mut hash: String = String::new();
    if let Some(pos) = projects.iter().position(|x| x.project_name == app_name) {
        let app = projects.remove(pos);
        hash = String::from_utf8_lossy(&app.project_hash).to_string();
    }
    let mut nxs: NXS = NXS {
        header: NXSHeader {
            magic_number: [0u8; 4],
            format_version: [0u8; 6],
            project_count: 0,
            reserved: 0,
        },
        projects: ProjectList { entries: vec![] },
    };
    nxs::update_project_entries(&mut nxs, projects);
    nxp::delete_nxp(&hash);
    lrncore::logs::info_log("Successfully remove project from list");
}

fn remove_project_from_storage() {
    let mut projects = nxs::get_all_project();
    inquire::set_global_render_config(utils::get_render_config());
    let app_name = Text::new("Enter the name of the project:")
        .prompt()
        .expect("Failed to read project id");
    let mut hash: String = String::new();
    if let Some(pos) = projects.iter().position(|app| app.project_name == app_name) {
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
    nxp::parse_nxp_file(&format!(".nxfs/projects/{}", &hash), &mut nxp);
    Command::new("rm")
        .arg("-rf")
        .arg(nxp.content.location)
        .spawn()
        .expect("Failed to delete the directory of the project");
    let mut nxs: NXS = NXS {
        header: NXSHeader {
            magic_number: [0u8; 4],
            format_version: [0u8; 6],
            project_count: 0,
            reserved: 0,
        },
        projects: ProjectList { entries: vec![] },
    };
    nxs::update_project_entries(&mut nxs, projects);
    nxp::delete_nxp(&hash);
    lrncore::logs::info_log("Successfully delete project from storage");
}
