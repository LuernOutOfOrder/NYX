mod build;
mod cleanup;
mod git;
mod projects;
mod update;
mod utils;

use std::{env, process::exit};

// Current version of NYX
// if modified and then running update command it will replace
// your current nyx installation with the newer version
const VERSION: &'static str = "0.9.93";

#[derive(Debug, Clone)]
enum Commands {
    Project { name: Option<String> },
    ProjectAdd,
    ProjectList,
    ProjectDelete,
    ProjectBuild,
    ProjectUpdate,
    Cleanup,
    GitStash,
    GitTag,
    GitReverse,
    Update,
    Help,
    Version,
}

fn main() {
    // let args = Args::parse();
    let args: Vec<String> = env::args().collect();

    if let Some(arg) = args.iter().last() {
        match arg.as_str().trim() {
            "-v" => {
                utils::command_usage(&nyx_version());
            }
            "--version" => {
                utils::command_usage(&nyx_version());
            }
            _ => {}
        }
    }

    let command = match args.get(1).map(|s| s.as_str()) {
        Some("project") => Commands::Project {
            name: args.get(2).cloned(),
        },
        Some("project-add") => Commands::ProjectAdd,
        Some("project-list") => Commands::ProjectList,
        Some("project-delete") => Commands::ProjectDelete,
        Some("project-build") => Commands::ProjectBuild,
        Some("project-update") => Commands::ProjectUpdate,
        Some("cleanup") => Commands::Cleanup,
        Some("git-stash") => Commands::GitStash,
        Some("git-tag") => Commands::GitTag,
        Some("git-reverse") => Commands::GitReverse,
        Some("update") => Commands::Update,
        Some("help") => Commands::Help,
        Some("version") => Commands::Version,
        _ => {
            usage_and_exit("Invalid command".to_string());
            return;
        }
    };

    match command {
        Commands::Project { name } => projects::new_project(name),
        Commands::ProjectAdd => projects::list::add_existing_project_to_list(),
        Commands::ProjectList => projects::list::list_projects(),
        Commands::ProjectDelete => projects::remove::select_remove_project(),
        Commands::ProjectBuild => build::build_current_project(),
        Commands::ProjectUpdate => projects::update_project(),
        Commands::Cleanup => cleanup::choose_cleanup(),
        Commands::GitStash => git::nyx_git_stash(),
        Commands::GitTag => git::nyx_git_tag(),
        Commands::GitReverse => git::nyx_git_revert(),
        Commands::Update => update::update_bin(),
        Commands::Help => utils::nyx_usage(),
        Commands::Version => utils::command_usage(&nyx_version()),
    }
}

fn usage_and_exit(msg: String) {
    if msg != "" {
        eprintln!("{}", msg);
    }

    utils::nyx_usage();

    exit(0);
}

pub fn nyx_version() -> String {
    let usage = format!("nyx {VERSION}");
    usage
}
