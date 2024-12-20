mod git;
mod projects;
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
    #[command(about = "Generate new project")]
    Project { name: String },
    #[command(about = "Add an existing project to the projects list")]
    ProjectAdd,
    #[command(about = "List all projects")]
    ProjectList,
    #[command(about = "Remove project from list or completely")]
    ProjectDelete,
    #[command(about = "Build the current project in working directory")]
    ProjectBuild,
    #[command(about = "Update specified project properties")]
    ProjectUpdate,
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
        Commands::Project { name } => projects::new_project(name),
        Commands::ProjectAdd => projects::add_existing_project_to_list(),
        Commands::ProjectList => projects::list_projects(),
        Commands::ProjectDelete => projects::select_remove_project(),
        Commands::ProjectBuild => build::build_current_project(),
        Commands::ProjectUpdate => projects::update_one_project(),
        Commands::Cleanup => cleanup::choose_cleanup(),
        Commands::GitStash => git::nyx_git_stash(),
        Commands::GitTag => git::nyx_git_tag(),
        Commands::GitReverse => git::nyx_git_revert(),
        Commands::Update => update::update_bin(),
    }
}
