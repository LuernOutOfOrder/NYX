mod cleanup;
pub mod gh;
mod git;
mod health;
pub mod logs;
pub mod macros;
mod projects;
mod update;
mod utils;
use crate::projects::todo;
use lrncore::usage_exit::command_usage;

use std::env;

// Current version of NYX
// if modified and then running update command it will replace
// your current nyx installation with the newer version
const VERSION: &'static str = "1.8.1";

#[derive(Debug, Clone)]
enum Commands {
    Init,
    CatNxs,
    CatNxp { hash: Option<String> },
    Project,
    Cleanup,
    Git,
    Health,
    Update,
    Help,
    Version,
}

fn nyx_usage() -> &'static str {
    let usage = r"
Usage: nyx command [options]

A lightweight utility for efficient project management and useful tools.

Commands:
    init            Initialize NYX data
    cat-nxs         Emit NXS object content
    cat-nxp         Emit specified NXP object content
    project         Manage project-related tasks
    cleanup         Cleanup all unused files
    git             Git command wrapped in a simplified interface
    health          Display current development system health
    update          Update the current version of NYX
    help            Show this help message

Options:

    -h, --help      Show command usage
    -v, --version   Show the current version of NYX
";

    return usage;
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if let Some(arg) = args.iter().last() {
        match arg.as_str().trim() {
            "-v" => {
                command_usage(&nyx_version());
            }
            "--version" => {
                command_usage(&nyx_version());
            }
            _ => {}
        }
    }

    let command = match args.get(1).map(|s| s.as_str()) {
        Some("init") => Commands::Init,
        Some("cat-nxs") => Commands::CatNxs,
        Some("cat-nxp") => Commands::CatNxp {
            hash: args.get(2).cloned(),
        },
        Some("project") => Commands::Project,
        Some("cleanup") => Commands::Cleanup,
        Some("git") => Commands::Git,
        Some("health") => Commands::Health,
        Some("update") => Commands::Update,
        Some("help") => Commands::Help,
        Some("version") => Commands::Version,
        _ => {
            lrncore::usage_exit::usage_and_exit("Invalid command", nyx_usage());
            return;
        }
    };

    match command {
        Commands::Init => projects::nxs::create_data(),
        Commands::CatNxs => projects::nxs::cat_nxs(),
        Commands::CatNxp { hash } => projects::nxp::cat_nxp(hash),
        Commands::Project => projects::project_command(),
        Commands::Cleanup => cleanup::choose_cleanup(),
        Commands::Git => git::git_command(),
        Commands::Health => health::dev_env_health(),
        Commands::Update => update::update_bin(),
        Commands::Help => command_usage(nyx_usage()),
        Commands::Version => command_usage(&nyx_version()),
    }
}

pub fn nyx_version() -> String {
    let usage = format!("nyx {VERSION}");
    usage
}
