use std::process::Command;

use crate::env;
use crate::utils;

use lrncore::usage_exit::command_usage;

fn git_help() -> String {
    let usage = r"
Usage: nyx git [subcommand] [arguments] [options]

Subcommands:
    stash       Stash with a message
    tag         Create a new tag and push to origin
    summarize   Summarize current git repository with last commit, all commits, all branches and stash list
    all-commit  Shortlog all commits with number and users
    last-commit Show last 4 detailed commits

Options:
    -h, --help      Show this help message
";

    usage.to_string()
}

pub fn git_command() {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 2 {
        command_usage(&git_help());
    }
    match args[2].as_str() {
        "stash" => nyx_git_stash(),
        "tag" => nyx_git_tag(),
        "summarize" => git_summarize(),
        "all-commit" => show_all_commit(),
        "last-commit" => show_last_commit_with_stat(),
        _ => {
            command_usage(&git_help());
        }
    }
}

fn nyx_git_stash() {
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

fn nyx_git_tag() {
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

pub fn git_init() {
    let mut git_init = Command::new("git")
        .arg("init")
        .spawn()
        .expect("Failed to create the remote repository");
    let wait_git_init = git_init.wait().expect("Failed to wait the gh command");
    if !wait_git_init.success() {
        panic!();
    }
}

fn git_summarize() {
    println!("Last commits with stat: \n");
    show_last_commit_with_stat();
    println!("All commits by all users: ");
    show_all_commit();
    println!("All branches: ");
    show_all_branch();
    println!("Stash: ");
    show_stash();
}

fn show_all_commit() {
    let shortlog = Command::new("git")
        .arg("shortlog")
        .arg("--summary")
        .arg("--numbered")
        .arg("--all")
        .arg("--no-merges")
        .output()
        .expect("Failed to call the git shortlog command");
    println!("{}", String::from_utf8_lossy(&shortlog.stdout));
}

fn show_last_commit_with_stat() {
    let commits = Command::new("git")
        .arg("log")
        .arg("-4")
        .arg("--stat")
        .output()
        .expect("Failed to call the git log command");
    println!("{}", String::from_utf8_lossy(&commits.stdout));
}

fn show_all_branch() {
    let branches = Command::new("git")
        .arg("branch")
        .output()
        .expect("Failed to call the git shortlog command");
    println!("{}", String::from_utf8_lossy(&branches.stdout));
}

fn show_stash() {
    let list = Command::new("git")
        .arg("stash")
        .arg("list")
        .output()
        .expect("Failed to call the git shortlog command");
    println!("{}", String::from_utf8_lossy(&list.stdout));
}
