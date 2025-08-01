use crate::nxfs::config::LogLevel;
use crate::plugins::parser::parse_plugin_file;
use crate::plugins::run_init_command;
use crate::utils::{self, log};
use std::{fs, process::exit};
pub mod list;
pub mod update;
use copy::copy_command;
use delete::select_remove_project;
use list::{add_existing_project_to_list, list_projects};
use lrncore::path::change_work_dir;
use std::env;
use todo::choose_todo;
use update::update_project_properties;
pub mod delete;
use crate::nxfs;
use lrncore::usage_exit::command_usage;
use nxfs::{nxp, nxs};
mod copy;
pub mod open;
pub mod todo;
use open::open_editor;

pub fn project_help() -> &'static str{
    (r"
Usage: nyx project [subcommand] [arguments] [options]

Subcommands:
    new         Create a new project
    open        Open your editor in project location
    add         Add an existing project to the list
    list        List all projects
    delete      Remove a project from the list
    update      Update project properties
    todo        Manage project todos
    copy        Copy a field of a specified project in clipboard

Options:
    -h, --help      Show this help message
    ") as _
}

pub fn project_command() {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 2 {
        command_usage(project_help());
    }
    match args[2].as_str() {
        "new" => {
            if args.len() <= 3 {
                log::log_from_log_level(LogLevel::Error, "Enter a new project name");
                exit(4);
            }
            let project_name = &args[3];
            new_project(project_name);
        }
        "open" => {
            let project_name: String;
            if args.len() <= 3 {
                project_name = "".to_owned();
                open_editor(&project_name);
            } else {
                project_name = args[3].clone();
                open_editor(&project_name);
            }
        }
        "add" => add_existing_project_to_list(),
        "list" => list_projects(),
        "delete" => select_remove_project(),
        "update" => update_project_properties(),
        "todo" => choose_todo(),
        "copy" => copy_command(),
        _ => {
            command_usage(project_help());
        }
    }
}

// new project
fn new_project(name: &str) {
    let args: Vec<String> = env::args().collect();
    if let Some(arg) = args.iter().last() {
        match arg.as_str().trim() {
            "-h" => {
                command_usage(project_help());
            }
            "--help" => {
                command_usage(project_help());
            }
            _ => {}
        }
    }

    inquire::set_global_render_config(utils::get_render_config());
    let option_select =
        utils::get_select_project_option("Which tech do you want to use ?");

    match fs::create_dir(name) {
        Ok(_) => println!("Directory created successfully"),
        Err(e) => println!("Failed to create directory: {e}"),
    }
    lrncore::path::change_work_dir(name);
    match option_select {
        Ok(choice) => new_project_by_choice(&choice, name),
        Err(_) => println!("There was an error, please try again"),
    }
}

fn new_project_by_choice(tech: &str, name: &str) {
    println!("{:?}", env::current_dir());
    let plugin = parse_plugin_file(tech).expect("Failed to load plugin");
    run_init_command(plugin);
    list::create_repo_or_not(tech);
}

