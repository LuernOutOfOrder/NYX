mod application;
mod git;
use clap::{Parser, Subcommand};
mod build;
mod cleanup;
mod update;
mod utils;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[derive(Debug)]
struct Args {
    #[command(subcommand)]
    cmd: Commands,
}

#[derive(Subcommand, Debug, Clone)]
enum Commands {
    #[command(about = "Generate new app")]
    App { name: String },
    #[command(about = "Add an existing app to the applications list")]
    AppAdd,
    #[command(about = "List all applications")]
    AppList,
    #[command(about = "Remove application from list or completely")]
    AppDelete,
    #[command(about = "Build the current project in working directory")]
    AppBuild,
    #[command(about = "Cleanup all unused files")]
    Cleanup,
    #[command(about = "Stash with message")]
    GitStash,
    #[command(about = "Create a new tag and push it to the origin branch")]
    GitTag,
    #[command(about = "Revert to the specified commit")]
    GitReverse,
    #[command(about = "Update the current version of NYX")]
    Update,
}

fn main() {
    let args = Args::parse();

    match args.cmd {
        Commands::App { name } => application::new_project(name),
        Commands::AppAdd => application::add_existing_app_to_list(),
        Commands::AppList => application::list_app(),
        Commands::AppDelete => application::select_remove_app(),
        Commands::AppBuild => build::build_current_project(),
        Commands::Cleanup => cleanup::choose_cleanup(),
        Commands::GitStash => git::nyx_git_stash(),
        Commands::GitTag => git::nyx_git_tag(),
        Commands::GitReverse => git::nyx_git_revert(),
        Commands::Update => update::update_bin(),
    }
}
