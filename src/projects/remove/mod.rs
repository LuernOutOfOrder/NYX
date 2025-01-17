use crate::projects::{self};
use inquire::{InquireError, Select, Text};
use std::{fs, process::Command};

use crate::utils;

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
    let app_id = Text::new("Enter the id of the project:")
        .prompt()
        .expect("Failed to read project id");
    // if an index match the given data, remove it from the vector
    if let Some(pos) = projects.iter().position(|x| x.id == app_id) {
        projects.remove(pos);
    }
    let updated_data = projects::Data { project: projects };
    let save_json = serde_json::to_string(&updated_data).expect("Failed to serialize data");
    fs::write(app_data_path, save_json).expect("Failed to write updated data");
    println!("Successfully remove project from list");
}

fn remove_project_from_storage() {
    let app_data_path = utils::get_app_data();
    let mut projects = utils::get_app_vec();
    inquire::set_global_render_config(utils::get_render_config());
    let app_id = Text::new("Enter the id of the project:")
        .prompt()
        .expect("Failed to read project id");
    if let Some(pos) = projects.iter().position(|app| app.id == app_id) {
        let app = projects.remove(pos);
        let app_location = &app.location;
        Command::new("rm")
            .arg("-rf")
            .arg(app_location)
            .spawn()
            .expect("Failed to delete the directory of the project");
        println!("Successfully delete project from storage");
    }
    let updated_data = projects::Data { project: projects };
    let save_json = serde_json::to_string(&updated_data).expect("Failed to serialize data");
    fs::write(app_data_path, save_json).expect("Failed to write updated data");
    println!("Successfully remove project from list");
}
