mod cleanup;
mod doctor;
pub mod gh;
mod git;
pub mod logs;
pub mod macros;
mod projects;
mod update;
mod upgrade;
mod utils;
use crate::projects::todo;
use lrncore::usage_exit::command_usage;
pub mod nxfs;
use std::env;
mod health;
mod hello;
mod init;
pub mod plugins;

// Current version of NYX
// if modified and then running update command it will replace
// your current nyx installation with the newer version
const VERSION: &str = "2.11.0";
enum Commands {
    Init,
    CatNxs,
    CatNxp { hash: Option<String> },
    Project,
    Cleanup,
    Git,
    Doctor,
    Health,
    Hello,
    Update,
    Config,
    Plugins,
    Upgrade,
    Help,
    Version,
}

fn nyx_usage() -> &'static str {
    r"
Usage: nyx command [options]

A lightweight utility for efficient project management and useful tools.

Commands:
    init            Initialize NYX data
    cat-nxs         Emit NXS object content
    cat-nxp         Emit specified NXP object content
    project         Manage project-related tasks
    cleanup         Cleanup all unused files
    git             Git command wrapped in a simplified interface
    doctor          Display current NYX system health
    health          Display current user configure environment health
    update          Execute all specified user configuration commands to update multiple tools at once
    hello           Display helpful information about today
    upgrade         Update the current version of NYX
    config          Manage nyx configuration
    Plugins         Manage plugins 
    help            Show this help message

Options:

    -h, --help      Show command usage
    -v, --version   Show the current version of NYX
"
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
        Some("doctor") => Commands::Doctor,
        Some("health") => Commands::Health,
        Some("hello") => Commands::Hello,
        Some("update") => Commands::Update,
        Some("config") => Commands::Config,
        Some("plugins") => Commands::Plugins,
        Some("upgrade") => Commands::Upgrade,
        Some("help") => Commands::Help,
        Some("version") => Commands::Version,
        _ => {
            lrncore::usage_exit::usage_and_exit("Invalid command", nyx_usage());
            return;
        }
    };

    match command {
        Commands::Init => init::init_command(),
        Commands::CatNxs => nxfs::nxs::cat_nxs(),
        Commands::CatNxp { hash } => nxfs::nxp::cat_nxp(hash.as_deref()),
        Commands::Project => projects::project_command(),
        Commands::Cleanup => cleanup::choose_cleanup(),
        Commands::Git => git::git_command(),
        Commands::Doctor => doctor::doctor_health(),
        Commands::Health => health::health_command(),
        Commands::Hello => hello::hello_command(),
        Commands::Config => nxfs::config::config_command(),
        Commands::Plugins => plugins::plugins_command(),
        Commands::Update => update::update_command(),
        Commands::Upgrade => upgrade::upgrade_bin(),
        Commands::Help => command_usage(nyx_usage()),
        Commands::Version => command_usage(&nyx_version()),
    }
}

pub fn nyx_version() -> String {
    format!("nyx {VERSION}")
}
