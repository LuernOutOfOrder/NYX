use inquire::{InquireError, Select};
use std::env;
use std::fs;
use std::process::Command;
mod templates;

pub fn new_project(name: String) {
    let options: Vec<&str> = vec!["Node.js", "Python", "Golang", "Rust", "C++"];

    let ans: Result<&str, InquireError> =
        Select::new("Which tech do you want to use ?", options).prompt();

    match fs::create_dir(name.clone()) {
        Ok(_) => println!("Directory created successfully"),
        Err(e) => println!("Failed to create directory: {}", e),
    }
    change_work_dir(&name);
    match ans {
        Ok(choice) => new_app_by_choice(choice),
        Err(_) => println!("There was an error, please try again"),
    }
}

fn new_app_by_choice(tech: &str) {
    match tech {
        tech if tech == "Node.js" => new_nodejs_app(),
        _ => println!("please select a tech"),
    }
}

fn new_nodejs_app() {
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
    let tech = "Node.js";
    generate_gitignore(&tech);
    println!("Successfully generate the new Node.js application")
}

fn generate_gitignore(tech: &str) -> String {
    match tech {
        tech if tech == "Node.js" => templates::nodejs_gitignore_template(),
        _ => "please select a tech".to_string(),
    }
}

fn change_work_dir(dir: &String) {
    env::set_current_dir(&dir).expect("Failed to change directory");
}
