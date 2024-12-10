use crate::utils;
use inquire::{InquireError, Select};
use std::process::Command;

pub fn build_current_project() {
    inquire::set_global_render_config(utils::get_render_config());
    let options: Vec<&str> = vec!["Node.js", "Python", "Golang", "Rust"];

    let ans: std::result::Result<&str, InquireError> =
        Select::new("Which tech your current project is using ?", options).prompt();

    match ans {
        Ok(choice) => build_project_by_tech(choice),
        Err(_) => println!("There was an error, please try again"),
    }
}

fn build_project_by_tech(tech: &str) {
    match tech {
        tech if tech == "Node.js" => print!("not supported yet"),
        tech if tech == "Python" => print!("not supported yet"),
        tech if tech == "Golang" => build_golang_project(),
        tech if tech == "Rust" => build_rust_project(),
        _ => println!("please select a tech"),
    }
}

fn build_golang_project() {
    let mut go_build = Command::new("go")
        .arg("build")
        .arg(".")
        .spawn()
        .expect("Failed to execute the go build command");
    let wait_go_build = go_build
        .wait()
        .expect("Failed to wait the go build command");
    if !wait_go_build.success() {
        panic!("Error building the Golang project");
    }
    print!("successfully build the Golang project")
}

fn build_rust_project() {
    let mut cargo_build = Command::new("cargo")
        .arg("build")
        .spawn()
        .expect("Failed to execute the cargo build command");
    let wait_cargo_build = cargo_build
        .wait()
        .expect("Failed to wait the cargo build command");
    if !wait_cargo_build.success() {
        panic!("Error building the Rust project");
    }
    print!("successfully build the Rust project")
}
