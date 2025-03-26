use crate::gh;
use crate::projects::nxs;
use crate::utils;
use crate::vec_of_strings;
use inquire::{error::InquireError, Select};
use std::env;
use tabled::settings::Style;
use tabled::Table;

use super::nxp::{self, NXPContent};

pub fn project_list_help() -> String {
    let usage = r"
Usage: nyx project-list [options]

Options:

    -s, --short     List all project without Github and version data
    -h, --help      Show this help message
";

    return usage.to_string();
}

pub fn list_projects() {
    println!("Listing all projects...");
    let args: Vec<String> = env::args().collect();
    let projects = nxs::get_all_project_entries();
    let projects_short = nxs::get_all_short_project();
    let mut builder = Table::builder(&projects).index().name(None);
    if let Some(arg) = args.iter().last() {
        match arg.as_str().trim() {
            "-s" => {
                builder = Table::builder(&projects_short).index().name(None);
            }
            "--short" => {
                builder = Table::builder(&projects_short).index().name(None);
            }
            "-h" => {
                utils::command_usage(&project_list_help());
            }
            "--help" => {
                utils::command_usage(&project_list_help());
            }
            _ => {}
        }
    }
    let mut table = builder.build();
    table.with(Style::modern());
    println!("{}", table);
}

pub fn add_existing_project_to_list() {
    inquire::set_global_render_config(utils::get_render_config());
    let options = utils::get_tech_option();
    let ans: std::result::Result<String, InquireError> =
        Select::new("Which tech your project is using ?", options).prompt();
    match ans {
        Ok(choice) => create_repo_or_not(&choice),
        Err(_) => println!("There was an error, please try again"),
    }
}

pub fn create_repo_or_not(tech: &str) {
    inquire::set_global_render_config(utils::get_render_config());
    let choice: Vec<String> = vec_of_strings!["Yes".to_string(), "No".to_string()];
    let options = utils::get_select_option(
        "Do you want to create a new repository ?".to_string(),
        choice,
    )
    .unwrap();
    match options.as_str() {
        "Yes" => create_repo_add_to_list(tech),
        "No" => add_project_to_list(tech),
        _ => {
            println!("There was an error, please try again")
        }
    }
}

pub fn add_project_to_list(tech: &str) {
    let current_dir = utils::get_current_path();
    let app_name = current_dir.split("/").last().unwrap();
    let mut repository_user_input: String;
    repository_user_input = utils::prompt_message(
        "Enter the url of the github's repository of the project: ".to_string(),
        "Failed to get the user input".to_string(),
    );

    if repository_user_input == "".to_string() {
        repository_user_input = "No repository specified".to_string()
    }

    let mut github_project: String;

    github_project = utils::prompt_message(
        "Enter the url of the github project: ".to_string(),
        "Error getting the user input".to_string(),
    );

    if github_project == "".to_string() {
        github_project = "No github project specified".to_string()
    }

    let mut version: String;

    version = utils::prompt_message(
        "Enter the version of the project: ".to_string(),
        "Error getting the user input".to_string(),
    );

    if version == "".to_string() {
        version = "0.1.0".to_string();
    }

    let new_app: nxp::NXPContent = nxp::NXPContent {
        name: (app_name.to_string()),
        tech: (tech.to_string()),
        location: (current_dir),
        repository: repository_user_input,
        github_project: github_project,
        version: version,
    };
    nxp::create_new_nxp(new_app);
}

fn create_repo_add_to_list(tech: &str) {
    let current_dir = utils::get_current_path();
    let app_name = current_dir.split("/").last().unwrap();
    let choice = vec_of_strings!["public", "private", "internal"];
    let repository_visibility: String =
        utils::get_select_option("Select the repository visibility:".to_string(), choice).unwrap();
    gh::create_new_repo(app_name.to_string(), repository_visibility);
    let mut github_project: String;

    let repository: String = " https://github.com/ElouanDaCosta/".to_string() + app_name;
    github_project = utils::prompt_message(
        "Enter the url of the github project: ".to_string(),
        "Error getting the user input".to_string(),
    );

    if github_project == "".to_string() {
        github_project = "No github project specified".to_string()
    }

    let mut version: String;

    version = utils::prompt_message(
        "Enter the version of the project: ".to_string(),
        "Error getting the user input".to_string(),
    );

    if version == "".to_string() {
        version = "0.1.0".to_string();
    }

    let new_app: NXPContent = NXPContent {
        name: (app_name.to_string()),
        tech: (tech.to_string()),
        location: (current_dir),
        repository: repository,
        github_project: github_project,
        version: version,
    };
    nxp::create_new_nxp(new_app);
}
