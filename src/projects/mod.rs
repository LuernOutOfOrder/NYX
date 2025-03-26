use crate::utils;
use std::process::Command;
use std::{fs, process::exit};
pub mod list;
mod templates;
pub mod update;
use delete::select_remove_project;
use list::{add_existing_project_to_list, list_projects};
use serde::{Deserialize, Serialize};
use std::env;
use tabled::Tabled;
use todo::choose_todo;
use update::update_project_properties;
pub mod delete;
pub mod nxp;
pub mod nxs;
pub mod todo;

pub fn project_help() -> String {
    let usage = r"
Usage: nyx project [subcommand] [arguments] [options]

Subcommands:
    new        Create a new project
    add         Add an existing project to the list
    list        List all projects
    delete      Remove a project from the list
    update      Update project properties
    todo        Manage project todos

Options:
    -h, --help      Show this help message
";

    return usage.to_string();
}

#[derive(Deserialize, Serialize, Debug, Tabled, Clone, PartialEq)]
pub struct Project {
    pub id: String,
    pub name: String,
    pub tech: String,
    pub location: String,
    pub repository: String,
    pub github_project: String,
    pub version: String,
    pub todo: String,
}

#[derive(Tabled)]
pub struct ProjectShort {
    pub id: String,
    pub name: String,
    pub tech: String,
    pub location: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Data {
    pub project: Vec<Project>,
}

pub fn project_command() {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 2 {
        utils::command_usage(&project_help());
    }
    match args[2].as_str() {
        "new" => {
            if args.len() <= 3 {
                lrncore::logs::error_log("Enter a new project name");
                exit(1);
            }
            let project_name = &args[3];
            new_project(project_name.to_string());
        }
        "add" => add_existing_project_to_list(),
        "list" => list_projects(),
        "delete" => select_remove_project(),
        "update" => update_project_properties(),
        "todo" => choose_todo(),
        _ => {
            lrncore::logs::warning_log("Unknown command");
            utils::command_usage(&project_help());
        }
    }
}

// new project
fn new_project(name: String) {
    let args: Vec<String> = env::args().collect();
    if let Some(arg) = args.iter().last() {
        match arg.as_str().trim() {
            "-h" => {
                utils::command_usage(&project_help());
            }
            "--help" => {
                utils::command_usage(&project_help());
            }
            _ => {}
        }
    }

    inquire::set_global_render_config(utils::get_render_config());
    let option_select = utils::get_select_app_option("Which tech do you want to use ?".to_string());

    match fs::create_dir(name.clone()) {
        Ok(_) => println!("Directory created successfully"),
        Err(e) => println!("Failed to create directory: {}", e),
    }
    utils::change_work_dir(&name);
    match option_select {
        Ok(choice) => new_project_by_choice(&choice, &name),
        Err(_) => println!("There was an error, please try again"),
    }
}

fn new_project_by_choice(tech: &String, name: &str) {
    match tech {
        tech if tech == "Node.js" => new_nodejs_project(tech),
        tech if tech == "Python" => new_python_project(tech),
        tech if tech == "Golang" => new_golang_project(name, tech),
        tech if tech == "Rust" => new_rust_project(tech),
        _ => println!("please select a tech"),
    }
    list::create_repo_or_not(&tech);
}

// tech project

fn new_nodejs_project(tech: &str) {
    let mut npm = Command::new("npm")
        .arg("init")
        .arg("-y")
        .spawn()
        .expect("npm command failed to execute");
    let status = npm.wait().expect("Failed to wait on npm process");
    if !status.success() {
        panic!("Error init npm app");
    }
    let mut ts = Command::new("npm")
        .arg("install")
        .arg("--save-dev")
        .arg("typescript")
        .spawn()
        .expect("failed to install typescript");
    let ts_status = ts
        .wait()
        .expect("Failed to wait on typescript install process");
    if !ts_status.success() {
        panic!("Error installing typescript");
    }
    Command::new("touch")
        .arg("tsconfig.json")
        .spawn()
        .expect("failed to generate tsconfig.json");
    templates::new_gitignore(&tech);
    println!("Successfully generate the new Node.js project")
}

fn new_python_project(tech: &str) {
    let mut python3 = Command::new("python3")
        .arg("-m")
        .arg("venv")
        .arg("./")
        .spawn()
        .expect("Failed to generate the virtual environment of python");
    let venv_status = python3
        .wait()
        .expect("Failed to wait on the generation of venv");
    if !venv_status.success() {
        panic!("Error init python virtual environment")
    }
    templates::new_gitignore(&tech);
    println!("Successfully generate the new Python project");
}

fn new_golang_project(name: &str, tech: &str) {
    let mut go_init = Command::new("go")
        .arg("mod")
        .arg("init")
        .arg(name)
        .spawn()
        .expect("Failed to generate the new Golang project");
    let go_status = go_init
        .wait()
        .expect("Failed to wait on the generation of the Golang project");
    if !go_status.success() {
        panic!("Error init the Golang project");
    }
    templates::new_gitignore(&tech);
}

fn new_rust_project(tech: &str) {
    let mut cargo_init = Command::new("cargo")
        .arg("init")
        .spawn()
        .expect("Failed to init the new Rust project using cargo.");
    let cargo_status = cargo_init
        .wait()
        .expect("Failed to wait on the generation of the Rust project");
    if !cargo_status.success() {
        panic!("Error init the Rust project")
    }
    templates::new_gitignore(&tech);
}
