use crate::utils;
use inquire::{error::InquireError, Select};
use std::env;
use std::fs;
use std::process::Command;
use tabled::settings::Style;
mod templates;
use serde::Deserialize;
use tabled::{Table, Tabled};

#[derive(Deserialize, Debug, Tabled)]
struct Application {
    id: String,
    name: String,
    tech: String,
    location: String,
}

#[derive(Deserialize, Debug)]
struct Data {
    application: std::collections::HashMap<String, Application>,
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

pub fn list_app() {
    let app_data_path = utils::get_app_data();
    let json_data = fs::read_to_string(app_data_path).expect("Failed to read app data");
    let data: Data = serde_json::from_str(&json_data).expect("Invalid JSON");
    let mut applications: Vec<Application> = Vec::new();
    for (_, app) in &data.application {
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
    println!("Listing all application...");
    println!("{}", table);
}

fn change_work_dir(dir: &String) {
    env::set_current_dir(&dir).expect("Failed to change directory");
}
