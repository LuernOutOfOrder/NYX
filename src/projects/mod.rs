use crate::utils;
use inquire::Text;
use inquire::{error::InquireError, Select};
use std::fs;
use std::process::Command;
use tabled::settings::Style;
mod templates;
mod update;
use serde::{Deserialize, Serialize};
use tabled::{Table, Tabled};

#[derive(Deserialize, Serialize, Debug, Tabled, Clone, PartialEq)]
pub struct Project {
    pub id: String,
    pub name: String,
    pub tech: String,
    pub location: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Data {
    pub project: Vec<Project>,
}

// new project

pub fn new_project(name: String) {
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
    add_project_to_list(tech);
}

// update project

pub fn update_project() {
    update::update_project_properties();
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

// project listing

pub fn add_existing_project_to_list() {
    inquire::set_global_render_config(utils::get_render_config());
    let options = utils::get_tech_option();
    let ans: std::result::Result<String, InquireError> =
        Select::new("Which tech your project is using ?", options).prompt();
    match ans {
        Ok(choice) => add_project_to_list(&choice),
        Err(_) => println!("There was an error, please try again"),
    }
}

fn add_project_to_list(tech: &String) {
    let app_data_path = utils::get_app_data();
    let mut projects = utils::get_app_vec();
    let current_dir = utils::get_current_path();
    let app_name = current_dir.split("/").last().unwrap();
    let app_id = &app_name[..3];
    let new_app: Project = Project {
        id: (app_id.to_string().to_lowercase()),
        name: (app_name.to_string()),
        tech: (tech.to_string()),
        location: (current_dir),
    };
    projects.push(new_app.clone());
    let updated_data = Data { project: projects };
    let save_json = serde_json::to_string(&updated_data).expect("Failed to serialize data");
    fs::write(app_data_path, save_json).expect("Failed to write updated data");
}

pub fn list_projects() {
    println!("Listing all projects...");
    let projects = utils::get_app_vec();

    let builder = Table::builder(&projects).index().name(None);

    let mut table = builder.build();
    table.with(Style::modern());
    println!("{}", table);
}

// remove project

pub fn select_remove_project() {
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
    let app_data_path = utils::get_app_data();
    let mut projects = utils::get_app_vec();
    inquire::set_global_render_config(utils::get_render_config());
    let app_name = Text::new("Enter the name of the project:")
        .prompt()
        .expect("Failed to read project name");
    // if an index match the given data, remove it from the vector
    if let Some(pos) = projects.iter().position(|x| x.name == app_name) {
        projects.remove(pos);
    }
    let updated_data = Data { project: projects };
    let save_json = serde_json::to_string(&updated_data).expect("Failed to serialize data");
    fs::write(app_data_path, save_json).expect("Failed to write updated data");
    println!("Successfully remove project from list");
}

fn remove_project_from_storage() {
    let app_data_path = utils::get_app_data();
    let mut projects = utils::get_app_vec();
    inquire::set_global_render_config(utils::get_render_config());
    let app_name = Text::new("Enter the name of the project:")
        .prompt()
        .expect("Failed to read project name");
    if let Some(pos) = projects.iter().position(|app| app.name == app_name) {
        let app = projects.remove(pos);
        let app_location = &app.location;
        Command::new("rm")
            .arg("-rf")
            .arg(app_location)
            .spawn()
            .expect("Failed to delete the directory of the project");
        println!("Successfully delete project from storage");
    }
    let updated_data = Data { project: projects };
    let save_json = serde_json::to_string(&updated_data).expect("Failed to serialize data");
    fs::write(app_data_path, save_json).expect("Failed to write updated data");
    println!("Successfully remove project from list");
}
