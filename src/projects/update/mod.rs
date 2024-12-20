use std::process::Command;

use inquire::Text;

use crate::utils;

pub fn update_project() {
    let mut projects = utils::get_app_vec();
    inquire::set_global_render_config(utils::get_render_config());
    let app_name = Text::new("Enter the name of the project:")
        .prompt()
        .expect("Failed to read project name");
    // if an index match the given data,
    let mut throbber = utils::custom_throbber("Start updating project".to_string());
    throbber.start();
    if let Some(pos) = projects.iter().position(|app| app.name == app_name) {
        let app = projects.remove(pos);
        utils::change_work_dir(&app.location);
        update_by_tech(&app.tech);
        throbber.success("Successfully updated the project".to_string());
        throbber.end();
    }
}

fn update_by_tech(tech: &String) {
    match tech {
        tech if tech == "Golang" => update_golang_project(),
        _ => println!("no tech matching detected."),
    }
}

fn update_golang_project() {
    let mut go_get = Command::new("go")
        .arg("get")
        .arg("-u")
        .arg("./...")
        .spawn()
        .expect("Failed to execute the go get command");
    let get_wait = go_get.wait().expect("Failed to wait the go get command");
    if !get_wait.success() {
        panic!("Error running the go get -u command");
    }
    let mut go_tidy = Command::new("go")
        .arg("mod")
        .arg("tidy")
        .spawn()
        .expect("Failed to execute the go mod tidy");
    let tidy_wait = go_tidy.wait().expect("Failed to wait the go mod tidy");
    if !tidy_wait.success() {
        panic!("Failed to execute the go mod tidy command")
    }
}
