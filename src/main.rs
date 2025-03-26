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
    GitStash,
    GitTag,
    GitReverse,
    GitSummarize,
    Health,
    Update,
    Help,
    Version,
}

fn main() {
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
        Some("init") => Commands::Init,
        Some("cat-nxs") => Commands::CatNxs,
        Some("cat-nxp") => Commands::CatNxp {
            hash: args.get(2).cloned(),
        },
        Some("project") => Commands::Project,
        Some("cleanup") => Commands::Cleanup,
        Some("git-stash") => Commands::GitStash,
        Some("git-tag") => Commands::GitTag,
        Some("git-reverse") => Commands::GitReverse,
        Some("git-summarize") => Commands::GitSummarize,
        Some("health") => Commands::Health,
        Some("update") => Commands::Update,
        Some("help") => Commands::Help,
        Some("version") => Commands::Version,
        _ => {
            lrncore::usage_exit::usage_and_exit("Invalid command", utils::nyx_usage());
            return;
        }
    };

    match command {
        Commands::Init => projects::nxs::create_data(),
        Commands::CatNxs => projects::nxs::cat_nxs(),
        Commands::CatNxp { hash } => projects::nxp::cat_nxp(hash),
        Commands::Project => projects::project_command(),
        Commands::Cleanup => cleanup::choose_cleanup(),
        Commands::GitStash => git::nyx_git_stash(),
        Commands::GitTag => git::nyx_git_tag(),
        Commands::GitReverse => git::nyx_git_revert(),
        Commands::GitSummarize => git::git_summarize(),
        Commands::Health => health::dev_env_health(),
        Commands::Update => update::update_bin(),
        Commands::Help => lrncore::usage_exit::command_usage(utils::nyx_usage()),
        Commands::Version => utils::command_usage(&nyx_version()),
    }
}

pub fn nyx_version() -> String {
    let usage = format!("nyx {VERSION}");
    usage
}
