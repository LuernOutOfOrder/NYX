use std::process::Command;

use inquire::Text;

use crate::utils;

pub fn nyx_git_stash() {
    inquire::set_global_render_config(utils::get_render_config());
    let message = Text::new("Enter stash message")
        .prompt()
        .expect("Failed to read stash message");
    let mut stash = Command::new("git")
        .arg("stash")
        .arg("push")
        .arg("-m")
        .arg(&message)
        .spawn()
        .expect("Failed to stash local change");
    let stash_status = stash.wait().expect("Failed to wait stash command");
    if !stash_status.success() {
        panic!("Error stash local change");
    }
}
