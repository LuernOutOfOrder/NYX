mod application;
use clap::{Parser, Subcommand};

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
    App { value: String, types: String },
}

fn main() {
    let args = Args::parse();

    match args.cmd {
        Commands::App { value, types } => application::new_nodejs_app(value, types),
    }
}
