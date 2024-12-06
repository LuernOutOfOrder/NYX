use crate::utils;
use inquire::Text;
use inquire::{error::InquireError, Select};
use std::env;
use std::fs;
use std::process::Command;
use tabled::settings::Style;
mod templates;
use serde::{Deserialize, Serialize};
use tabled::{Table, Tabled};

#[derive(Deserialize, Serialize, Debug, Tabled, Clone, PartialEq)]
struct Application {
    id: String,
    name: String,
    tech: String,
    location: String,
}

#[derive(Deserialize, Serialize, Debug)]
struct Data {
    application: Vec<Application>,
}

pub fn new_project(name: String) {
    inquire::set_global_render_config(utils::get_render_config());
    let options: Vec<&str> = vec!["Node.js", "Python", "Golang", "Rust"];

    let ans: std::result::Result<&str, InquireError> =
        Select::new("Which tech do you want to use ?", options).prompt();

    match fs::create_dir(name.clone()) {
        Ok(_) => println!("Directory created successfully"),
        Err(e) => println!("Failed to create directory: {}", e),
    }
    change_work_dir(&name);
    match ans {
        Ok(choice) => new_app_by_choice(choice, &name),
        Err(_) => println!("There was an error, please try again"),
    }
}

fn new_app_by_choice(tech: &str, name: &str) {
    match tech {
        tech if tech == "Node.js" => new_nodejs_app(tech),
        tech if tech == "Python" => new_python_app(tech),
        tech if tech == "Golang" => new_golang_app(name, tech),
        tech if tech == "Rust" => new_rust_app(tech),
        _ => println!("please select a tech"),
    }
    add_app_to_list(tech);
}

fn new_nodejs_app(tech: &str) {
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
    println!("Successfully generate the new Node.js application")
}

fn new_python_app(tech: &str) {
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
    println!("Successfully generate the new Python application");
}

fn new_golang_app(name: &str, tech: &str) {
    let mut go_init = Command::new("go")
        .arg("mod")
        .arg("init")
        .arg(name)
        .spawn()
        .expect("Failed to generate the new Golang application");
    let go_status = go_init
        .wait()
        .expect("Failed to wait on the generation of the Golang application");
    if !go_status.success() {
        panic!("Error init the Golang application");
    }
    templates::new_gitignore(&tech);
}

fn new_rust_app(tech: &str) {
    let mut cargo_init = Command::new("cargo")
        .arg("init")
        .spawn()
        .expect("Failed to init the new Rust application using cargo.");
    let cargo_status = cargo_init
        .wait()
        .expect("Failed to wait on the generation of the Rust application");
    if !cargo_status.success() {
        panic!("Error init the Rust application")
    }
    templates::new_gitignore(&tech);
}

pub fn add_existing_app_to_list() {
    inquire::set_global_render_config(utils::get_render_config());
    let options: Vec<&str> = vec!["Node.js", "Python", "Golang", "Rust"];

    let ans: std::result::Result<&str, InquireError> =
        Select::new("Which tech your application is using ?", options).prompt();
    match ans {
        Ok(choice) => add_app_to_list(choice),
        Err(_) => println!("There was an error, please try again"),
    }
}

fn add_app_to_list(tech: &str) {
    let app_data_path = utils::get_app_data();
    let json_data = fs::read_to_string(app_data_path.clone()).expect("Failed to read app data");
    let data: Data = serde_json::from_str(&json_data).expect("Invalid JSON");
    let mut applications: Vec<Application> = Vec::new();
    let current_dir = utils::get_current_path();
    let app_name = current_dir.split("/").last().unwrap();
    let app_id = &app_name[..3];
    let new_app: Application = Application {
        id: (app_id.to_string()),
        name: (app_name.to_string()),
        tech: (tech.to_string()),
        location: (current_dir),
    };
    for app in &data.application {
        applications.push(Application {
            id: app.id.clone(),
            name: app.name.clone(),
            tech: app.tech.clone(),
            location: app.location.clone(),
        });
    }
    applications.push(new_app.clone());
    let updated_data = Data {
        application: applications,
    };
    let save_json = serde_json::to_string(&updated_data).expect("Failed to serialize data");
    fs::write(app_data_path, save_json).expect("Failed to write updated data");
}

pub fn list_app() {
    println!("Listing all applications...");
    let app_data_path = utils::get_app_data();
    let json_data = fs::read_to_string(app_data_path).expect("Failed to read app data");
    let data: Data = serde_json::from_str(&json_data).expect("Invalid JSON");
    let mut applications: Vec<Application> = Vec::new();
    for app in &data.application {
        applications.push(Application {
            id: app.id.clone(),
            name: app.name.clone(),
            tech: app.tech.clone(),
            location: app.location.clone(),
        });
    }

    let builder = Table::builder(&applications).index().name(None);

    let mut table = builder.build();
    table.with(Style::modern());
    println!("{}", table);
}

pub fn select_remove_app() {
    inquire::set_global_render_config(utils::get_render_config());
    let options: Vec<&str> = vec![
        "Remove application from applications list",
        "Delete completely the application",
        "Nothing",
    ];

    let ans: std::result::Result<&str, InquireError> =
        Select::new("What do you want to do ?", options).prompt();

    match ans {
        Ok(choice) => which_remove_app(choice),
        Err(_) => println!("There was an error, please try again"),
    }
}

fn which_remove_app(choice: &str) {
    match choice {
        choice if choice == "Remove application from applications list" => remove_app_from_list(),
        // choice if choice == "Python" => new_python_app(choice),
        _ => println!("please make a choice"),
    }
}

fn remove_app_from_list() {
    let app_data_path = utils::get_app_data();
    let json_data = fs::read_to_string(app_data_path.clone()).expect("Failed to read app data");
    let data: Data = serde_json::from_str(&json_data).expect("Invalid JSON");
    let mut applications: Vec<Application> = Vec::new();
    for app in &data.application {
        applications.push(Application {
            id: app.id.clone(),
            name: app.name.clone(),
            tech: app.tech.clone(),
            location: app.location.clone(),
        });
    }
    inquire::set_global_render_config(utils::get_render_config());
    let app_name = Text::new("Enter the name of the application:")
        .prompt()
        .expect("Failed to read stash message");
    let app = applications.iter().find(|&app| app.name == app_name);
    // if an index match the given data, remove it from the vector
    if let Some(pos) = applications.iter().position(|x| x == app.unwrap()) {
        applications.remove(pos);
    }
    let updated_data = Data {
        application: applications,
    };
    let save_json = serde_json::to_string(&updated_data).expect("Failed to serialize data");
    fs::write(app_data_path, save_json).expect("Failed to write updated data");
}

fn change_work_dir(dir: &String) {
    env::set_current_dir(&dir).expect("Failed to change directory");
}
