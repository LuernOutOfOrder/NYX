use std::{env, process::Command};

use crate::{nxfs::nxs, utils};
use inquire::{InquireError, Select};
use lrncore::usage_exit::command_usage;

pub fn cleanup_help() -> String {
    let usage = r"
Usage: nyx cleanup

Options:

    -h, --help      Show this help message
";

    return usage.to_string();
}

pub fn choose_cleanup() {
    let args: Vec<String> = env::args().collect();
    if let Some(arg) = args.iter().last() {
        match arg.as_str().trim() {
            "-h" => {
                command_usage(&cleanup_help());
            }
            "--help" => {
                command_usage(&cleanup_help());
            }
            _ => {}
        }
    }
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
    let mut prune_throbber = utils::custom_throbber("Prune all unused docker files".to_string());
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

// this function remove all build cache,
// node_modules, bin folder content of the
// project managed by nyx
fn prune_project_unused() {
    let projects = nxs::get_all_project_entries();
    println!("Cleaning up all projects by removing dependency folders (node_modules), compiled files (dist), and executable binaries (bin) that are no longer needed.");
    for i in &projects {
        // Node.js
        let node_module_path = i.location.to_string() + "/node_modules";
        let nodejs_dist_path = i.location.to_string() + "/dist";
        lrncore::path::change_work_dir(&i.location);
        if lrncore::path::path_exists(&node_module_path) {
            utils::fsys::rm_command(node_module_path);
            utils::fsys::rm_command(nodejs_dist_path);
        }
        // Golang
        let bin_folder = i.location.to_string() + "/bin/";
        if lrncore::path::path_exists(&bin_folder) {
            utils::fsys::rm_command(bin_folder);
        }
        // Rust
        let target_folder = i.location.to_string() + "/target";
        if lrncore::path::path_exists(&target_folder) {
            utils::fsys::rm_command(target_folder);
        }
    }
}
