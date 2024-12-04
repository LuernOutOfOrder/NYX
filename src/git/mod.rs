use std::process::Command;

use inquire::Text;

use crate::utils;

pub fn nyx_git_stash() {
    inquire::set_global_render_config(utils::get_render_config());
    let message = Text::new("Enter stash message:")
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

pub fn nyx_git_tag() {
    inquire::set_global_render_config(utils::get_render_config());
    let name = Text::new("Enter new tag name:")
        .prompt()
        .expect("Failed to read tag name");
    let mut new_tag = Command::new("git")
        .arg("tag")
        .arg(name)
        .spawn()
        .expect("Failed to create new tag");
    let tag_status = new_tag.wait().expect("Failed to wait Git tag command");
    if !tag_status.success() {
        panic!("Error creating the new tag");
    }
    let mut push_tag = Command::new("git")
        .arg("push")
        .arg("--tags")
        .spawn()
        .expect("Failed to push current tags to remote repository");
    let push_tag_status = push_tag.wait().expect("Failed to wait push tags commands");
    if !push_tag_status.success() {
        panic!("Error pushing the new tags to remote repository");
    }
}

pub fn nyx_git_revert() {
    inquire::set_global_render_config(utils::get_render_config());
    let commit_name = Text::new("Enter commit id:")
        .prompt()
        .expect("Failed to read commit id");
    let mut reverse = Command::new("git")
        .arg("reverse")
        .arg(commit_name)
        .spawn()
        .expect("Failed to execute reverse command");
    let reverse_status = reverse.wait().expect("Failed to wait the reverse command");
    if !reverse_status.success() {
        panic!("Error running the reverse commit command");
    }
}
