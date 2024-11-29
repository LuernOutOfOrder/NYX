use std::process::Command;

pub fn nyx_git_stash() {
    let mut stash = Command::new("git")
        .arg("stash")
        .spawn()
        .expect("Failed to stash local change");
    let stash_status = stash.wait().expect("Failed to wait stash command");
    if !stash_status.success() {
        panic!("Error stash local change");
    }
}
