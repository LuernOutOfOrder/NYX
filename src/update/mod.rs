use colored::Colorize;
use std::process::Command;
use throbber::Throbber;

use crate::utils;

pub fn update_bin() {
    let nyx_art = utils::nyx_ascii_art();
    // throbber
    let mut building_throbber = Throbber::new()
        .message("Building latest NYX binary...".to_string())
        .frames(&throbber::ROTATE_F);
    let mut update_throbber = Throbber::new()
        .message("Updating NYX...".to_string())
        .frames(&throbber::ROTATE_F);
    println!("{}", nyx_art.truecolor(138, 43, 226));
    building_throbber.start();

    // nyx version
    let nyx_current_version = Command::new("nyx")
        .arg("version")
        .output()
        .expect("Failed to get the current version of NYX");
    let nyx_target_build_location = utils::env::get_nyx_env_var() + "/target/release";
    lrncore::path::change_work_dir(&nyx_target_build_location);
    let mut build_target = Command::new("cargo")
        .arg("build")
        .arg("--release")
        .spawn()
        .expect("Failed to build the target binary");
    let wait_build_target = build_target
        .wait()
        .expect("Failed to wait the cargo build command");
    if !wait_build_target.success() {
        building_throbber.fail("Error building latest binary".to_string());
        panic!();
    }
    building_throbber.success("Successfully build latest binary".to_string());
    building_throbber.end();
    let nyx_latest_version = Command::new("./nyx")
        .arg("version")
        .output()
        .expect("Failed to get the current version of NYX");
    if String::from_utf8(nyx_latest_version.stdout) != String::from_utf8(nyx_current_version.stdout)
    {
        println!("A new version of NYX has been found");
        update_throbber.start();
        lrncore::path::change_work_dir(&utils::env::get_nyx_env_var());
        let mut cargo_install = Command::new("cargo")
            .arg("install")
            .arg("--path")
            .arg(".")
            .spawn()
            .expect("Failed to update to the latest version");
        let wait_cargo_install = cargo_install
            .wait()
            .expect("Failed to wait the cargo install command");
        if !wait_cargo_install.success() {
            update_throbber.fail("Failed to update NYX to the latest version".to_string());
            panic!("Failed to update NYX");
        }
        update_throbber.success("Successfully update NYX".to_string());
        update_throbber.end();
    } else {
        println!("You already have the latest version of NYX!");
    }
}
