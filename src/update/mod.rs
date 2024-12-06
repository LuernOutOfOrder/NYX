use colored::Colorize;
use std::{process::Command, thread, time::Duration};
use throbber::Throbber;

use crate::utils;

pub fn update_bin() {
    let nyx_ascii_art = r"         
 _                         
( (    /||\     /||\     /|
|  \  ( |( \   / )( \   / )
|   \ | | \ (_) /  \ (_) / 
| (\ \) |  \   /    ) _ (  
| | \   |   ) (    / ( ) \ 
| )  \  |   | |   ( /   \ )
|/    )_)   \_/   |/     \|

"
    .truecolor(138, 43, 226);
    let mut throbber = Throbber::new()
        .message("Updating NYX...".to_string())
        .frames(&throbber::ROTATE_F);
    println!("{}", nyx_ascii_art);
    let nyx_current_version = Command::new("nyx")
        .arg("--version")
        .output()
        .expect("Failed to get the current version of NYX");
    let nyx_target_build_location = utils::get_nyx_env_var() + "/target/debug";
    utils::change_work_dir(&nyx_target_build_location);
    let nyx_latest_version = Command::new("nyx")
        .arg("--version")
        .output()
        .expect("Failed to get the current version of NYX");
    if String::from_utf8(nyx_latest_version.stdout) == String::from_utf8(nyx_current_version.stdout)
    {
        throbber.start();
        let nyx_bin = nyx_target_build_location + "/nyx";
        let move_latest_bin = Command::new("mv").arg()
        throbber.success("Success".to_string());
        throbber.end();
        println!("A new version of NYX has been found");
        println!("Updating NYX...");

    } else {
        println!("You already have the latest version of NYX!");
    }
}
