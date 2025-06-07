use crate::gh;
use crate::nxfs;
use crate::nxfs::config::LogLevel;
use crate::nxfs::nxs;
use crate::utils;
use crate::utils::get_select_project_option;
use crate::utils::log;
use lrncore::usage_exit::command_usage;
use std::env;
use std::process::exit;
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

    usage.to_string()
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
                command_usage(&project_list_help());
            }
            "--help" => {
                command_usage(&project_list_help());
            }
            _ => {}
        }
    }
    let mut table = builder.build();
    table.with(Style::modern());
    println!("{table}");
}

pub fn add_existing_project_to_list() {
    inquire::set_global_render_config(utils::get_render_config());
    let ans = get_select_project_option("Which tech your project is using ?");
    match ans {
        Ok(choice) => create_repo_or_not(&choice),
        Err(_) => println!("There was an error, please try again"),
    }
}

pub fn create_repo_or_not(tech: &str) {
    inquire::set_global_render_config(utils::get_render_config());
    let choice: Vec<&str> = vec!["Yes", "No"];
    let options = utils::get_select_option(
        "Do you want to create a new repository ?",
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
    let current_dir = lrncore::path::get_current_path();
    let app_name = current_dir.split("/").last().unwrap().to_lowercase();
    let mut repository_user_input: String;
    repository_user_input = utils::prompt_message(
        "Enter the url of the github's repository of the project: ",
        "Failed to get the user input",
    );

    if repository_user_input.is_empty() {
        repository_user_input = "No repository specified".to_owned()
    }

    let mut github_project: String;

    github_project = utils::prompt_message(
        "Enter the url of the github project: ",
        "Error getting the user input",
    );

    if github_project.is_empty() {
        github_project = "No github project specified".to_owned()
    }

    let mut version: String;

    version = utils::prompt_message(
        "Enter the version of the project: ",
        "Error getting the user input",
    );
    if version.is_empty() {
        version = "0.1.0".to_owned();
    }

    let new_app: nxp::NXPContent = nxp::NXPContent {
        name: (app_name.to_owned()),
        tech: (tech.to_owned()),
        location: (current_dir),
        repository: repository_user_input,
        github_project,
        version,
    };
    nxp::create_new_nxp(new_app);
}

fn create_repo_add_to_list(tech: &str) {
    let current_dir = lrncore::path::get_current_path();
    let app_name = current_dir.split("/").last().unwrap();
    let choice = vec!["public", "private", "internal"];
    let repository_visibility: String =
        utils::get_select_option("Select the repository visibility:", choice).unwrap();
    gh::create_new_repo(app_name, &repository_visibility);
    let mut github_project: String;
    let config = nxfs::config::parse_config_file().expect("Failed to parse the nyx config file");
    let user_github_url = config.git.profile_url;
    if user_github_url.is_empty() {
        log::log_from_log_level(
            LogLevel::Error,
            "No github url was specified. Please enter one in config file.",
        );
        exit(4);
    }
    let repository: String = user_github_url + app_name;
    github_project = utils::prompt_message(
        "Enter the url of the github project: ",
        "Error getting the user input",
    );

    if github_project.is_empty() {
        github_project = "No github project specified".to_owned()
    }

    let mut version: String;

    version = utils::prompt_message(
        "Enter the version of the project: ",
        "Error getting the user input",
    );

    if version.is_empty() {
        version = "0.1.0".to_owned();
    }

    let new_app: NXPContent = NXPContent {
        name: (app_name.to_owned()),
        tech: (tech.to_owned()),
        location: (current_dir),
        repository,
        github_project,
        version,
    };
    nxp::create_new_nxp(new_app);
}
