use crate::nxfs::config::LogLevel;
use crate::projects::nxp;
use crate::projects::{
    nxp::{NXPContent, NXPHeader, NXP},
    nxs::{self, NXSHeader, ProjectList, NXS},
};
use inquire::{InquireError, Select, Text};
use lrncore::usage_exit::command_usage;
use std::process::exit;
use std::{env, process::Command};

use crate::utils::{self, log};

pub fn project_delete_help() -> String {
    let usage = r"
Usage: nyx project-delete [options]

Options:

    -h, --help      Show this help message
";

    usage.to_string()
}

pub fn select_remove_project() {
    let args: Vec<String> = env::args().collect();
    if let Some(arg) = args.iter().last() {
        match arg.as_str().trim() {
            "-h" => {
                command_usage(&project_delete_help());
            }
            "--help" => {
                command_usage(&project_delete_help());
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
        "Remove project from projects list" => remove_project_from_list(),
        "Delete completely the project" => remove_project_from_storage(),
        _ => println!("please make a choice"),
    }
}

fn remove_project_from_list() {
    let mut projects = nxs::get_all_project();
    inquire::set_global_render_config(utils::get_render_config());
    let app_name = Text::new("Enter the name of the project:")
        .prompt()
        .expect("Failed to read project id");
    let confirm = utils::prompt::confirm_prompt(
        "Are you sure you want to remove this project from NYX ?",
        "You will lose all data store in NYX storage",
    );
    if !confirm {
        return;
    }
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
    log::log_from_log_level(LogLevel::Info, "Successfully remove project from list");
}

fn remove_project_from_storage() {
    let mut projects = nxs::get_all_project();
    inquire::set_global_render_config(utils::get_render_config());
    let app_name = Text::new("Enter the name of the project:")
        .prompt()
        .expect("Failed to read project id");
    let confirm = utils::prompt::confirm_prompt_safe_mode(
        "Are you sure you want to completely delete this project?",
        "It will be completely deleted from disk",
    );
    if !confirm {
        return;
    }
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
        },
    };
    nxp::parse_nxp_file(&format!(".nxfs/projects/{}/content", &hash), &mut nxp);
    let mut rm = Command::new("rm")
        .arg("-rf")
        .arg(nxp.content.location)
        .spawn()
        .expect("Failed to delete the directory of the project");
    let rm_wait = rm.wait().expect("Failed to wait rm command");
    if !rm_wait.success() {
        log::log_from_log_level(LogLevel::Error, "Failed to execute rm command");
        exit(1);
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
    log::log_from_log_level(LogLevel::Info, "Successfully delete project from storage");
}
