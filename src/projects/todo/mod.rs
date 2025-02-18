use inquire::{InquireError, Select};
use std::env;
mod parse;
use crate::utils;

pub fn todo_help() -> String {
    let usage = r"
Usage: nyx project-todo

Options:

    -h, --help      Show this help message
";

    return usage.to_string();
}

pub fn choose_todo() {
    let args: Vec<String> = env::args().collect();
    if let Some(arg) = args.iter().last() {
        //todo
        // flag to list or add directly to the todo
        match arg.as_str().trim() {
            "-h" => {
                utils::command_usage(&todo_help());
            }
            "--help" => {
                utils::command_usage(&todo_help());
            }
            _ => {}
        }
    }
    inquire::set_global_render_config(utils::get_render_config());
    let options: Vec<&str> = vec!["Add to to-do", "Show to-do list"];

    let ans: std::result::Result<&str, InquireError> =
        Select::new("What do you want to do ?", options).prompt();

    match ans {
        Ok(choice) => which_todo(choice),
        Err(_) => println!("There was an error, please try again"),
    }
}

fn which_todo(choice: &str) {
    match choice {
        choice if choice == "Add to to-do" => add_todo(),
        choice if choice == "Show to-do list" => show_todo(),
        _ => println!("please make a choice"),
    }
}

fn add_todo() {
    // utils::change_work_dir(&utils::get_current_path());
    parse::parse_todo_string();
}

fn show_todo() {
    println!("haha")
}
