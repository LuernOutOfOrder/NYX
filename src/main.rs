mod about;
mod application;
use clap::{Parser, Subcommand};
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
    App {
        name: String,
    },
    About,
}

fn main() {
    let args = Args::parse();

    match args.cmd {
        Commands::App { name } => application::new_project(name),
        Commands::About => about::about(),
    }
}
