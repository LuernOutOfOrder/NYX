use std::process::Command;

use inquire::{InquireError, Select};
use throbber::Throbber;

use crate::utils;

pub fn choose_cleanup() {
    inquire::set_global_render_config(utils::get_render_config());
    let options: Vec<&str> = vec![
        "Remove all docker unused files",
        "Remove all projects build caches and files",
        "Nothing",
    ];

    let ans: std::result::Result<&str, InquireError> =
        Select::new("What do you want to do ?", options).prompt();

    match ans {
        Ok(choice) => which_remove_files(choice),
        Err(_) => println!("There was an error, please try again"),
    }
}

fn which_remove_files(choice: &str) {
    match choice {
        choice if choice == "Remove all docker unused files" => prune_docker_unused(),
        choice if choice == "Remove all projects build caches and files" => prune_project_unused(),
        _ => println!("please make a choice"),
    }
}

fn prune_docker_unused() {
    let mut prune_throbber = Throbber::new()
        .message("Prune all unused docker files".to_string())
        .frames(&throbber::ROTATE_F);
    prune_throbber.start();
    // docker builder
    let mut docker_builder = Command::new("docker")
        .arg("builder")
        .arg("prune")
        .arg("-a")
        .arg("-f")
        .spawn()
        .expect("Failed to prune the build cache");
    let wait_docker_builder = docker_builder
        .wait()
        .expect("Failed to wait the docker builder command");
    if !wait_docker_builder.success() {
        prune_throbber.fail("Error prune the docker builder cache".to_string());
        panic!();
    }
    // docker volumes
    let mut docker_volumes = Command::new("docker")
        .arg("volume")
        .arg("prune")
        .arg("-a")
        .arg("-f")
        .spawn()
        .expect("Failed to prune the unused docker volumes");
    let wait_docker_volumes = docker_volumes
        .wait()
        .expect("Failed to wait the docker volume prune command");
    if !wait_docker_volumes.success() {
        prune_throbber.fail("Error prune the unused docker volume".to_string());
        panic!();
    }
    // docker images
    let mut docker_images = Command::new("docker")
        .arg("image")
        .arg("prune")
        .arg("-a")
        .arg("-f")
        .spawn()
        .expect("Failed to prune the docker image");
    let wait_docker_images = docker_images
        .wait()
        .expect("Failed to wait the docker image prune command");
    if !wait_docker_images.success() {
        prune_throbber.fail("Error prune the unused docker image".to_string());
        panic!();
    }
    prune_throbber.success("Successfully prune all unused docker files".to_string());
    prune_throbber.end();
}

fn prune_project_unused() {}
