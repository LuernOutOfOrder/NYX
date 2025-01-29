use std::process::Command;

use crate::utils;

pub fn nyx_git_stash() {
    let message = utils::prompt_message(
        "Enter stash message: ".to_string(),
        "Failed to read stash message".to_string(),
    );
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
    let name = utils::prompt_message(
        "Enter new tag name:".to_string(),
        "Failed to read tag name".to_string(),
    );
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
    let commit_name = utils::prompt_message(
        "Enter commit id: ".to_string(),
        "Failed to read commit id".to_string(),
    );
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

pub fn git_init() {
    let mut git_init = Command::new("git")
        .arg("init")
        .spawn()
        .expect("Failed to create the remote repostory");
    let wait_git_init = git_init.wait().expect("Failed to wait the gh command");
    if !wait_git_init.success() {
        panic!();
    }
}
