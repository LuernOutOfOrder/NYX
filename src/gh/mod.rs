use crate::git;
use std::process::Command;

pub fn create_new_repo(name: String, visibility: String) {
    git::git_init();
    let repo_visibility = "--".to_string() + &visibility;
    let source = "--source=.";
    let mut gh_repo = Command::new("gh")
        .arg("repo")
        .arg("create")
        .arg(name)
        .arg(repo_visibility)
        .arg(source)
        .arg("--remote=upstream")
        .spawn()
        .expect("Failed to create the remote repostory");
    let wait_gh_repo = gh_repo.wait().expect("Failed to wait the gh command");
    if !wait_gh_repo.success() {
        panic!();
    }
}
