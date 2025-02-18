use inquire::{InquireError, Select};
use std::env;
use tabled::settings::Style;
use tabled::Table;

mod parse;

use crate::{projects, utils};
use std::fs;

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
        choice if choice == "Add to to-do" => update_todo_list(),
        choice if choice == "Show to-do list" => show_todo(),
        _ => println!("please make a choice"),
    }
}

fn update_todo_list() {
    let new_todo = utils::prompt_message(
        "Enter new todo".to_string(),
        "Error getting user input".to_string(),
    );
    let app_data_path = utils::get_app_data();
    let mut projects = utils::get_app_vec();
    let get_current_workdir = utils::get_current_path();
    if let Some(pos) = projects
        .iter()
        .position(|app| app.location == get_current_workdir)
    {
        let app = projects.remove(pos);
        let todo_vec = parse::parse_todo(app.todo.clone());
        let updated_toto_vec = add_new_todo(todo_vec, &new_todo);
        let stringify_updated_todo_vec = parse::stringify_todo(updated_toto_vec);
        let mut updated_app = app;
        updated_app.todo = stringify_updated_todo_vec;
        projects.push(updated_app);
        let update_data = projects::Data { project: projects };
        let save_json = serde_json::to_string(&update_data).expect("Failed to serialize data");
        fs::write(app_data_path, save_json).expect("Failed to write updated data");
    }
}

fn show_todo() {
    let app_data_path = utils::get_app_data();
    let mut projects = utils::get_app_vec();
    let get_current_workdir = utils::get_current_path();
    if let Some(pos) = projects
        .iter()
        .position(|app| app.location == get_current_workdir)
    {
        let app = projects.remove(pos);
        let todo_vec = parse::parse_todo(app.todo.clone());
        let mut builder = Table::builder(&todo_vec).index().name(None);
        let mut table = builder.build();
        table.with(Style::modern());
        println!("{}", table);
    }
}

fn add_new_todo(mut todo_vec: Vec<String>, new_todo: &str) -> Vec<String> {
    todo_vec.push(new_todo.to_string());
    todo_vec
}
