use inquire::{InquireError, Select};
use serde::{Deserialize, Serialize};
use std::env;
use std::process::exit;
use tabled::Table;
use tabled::{settings::Style, Tabled};

mod parse;

use crate::{logs, projects, utils, vec_of_strings};
use std::fs;

#[derive(Serialize, Tabled, Deserialize, Clone, Debug)]
pub struct Todo {
    pub id: u8,
    pub note: String,
    pub status: String,
}

#[derive(Serialize, Debug)]
pub struct TodoData {
    todos: Vec<Todo>,
}

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
    let options: Vec<&str> = vec![
        "Add to to-do",
        "Show to-do list",
        "Clear the to-do list",
        "Remove one to-do",
        "Update one to-do status",
    ];

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
        choice if choice == "Clear the to-do list" => clear_todo(),
        choice if choice == "Remove one to-do" => remove_todo(),
        choice if choice == "Update one to-do status" => update_todo_status(),
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
        logs::info_log("Successfully added the new to-do".to_string());
    }
}

fn show_todo() {
    let mut projects = utils::get_app_vec();
    let get_current_workdir = utils::get_current_path();
    if let Some(pos) = projects
        .iter()
        .position(|app| app.location == get_current_workdir)
    {
        let app = projects.remove(pos);
        let todo_vec = parse::parse_todo(app.todo.clone());
        let parse_todo_vec: Vec<Todo> = todo_vec
            .iter()
            .map(|todo| serde_json::from_str(todo).expect("Failed to parse todo"))
            .collect();
        let builder = Table::builder(&parse_todo_vec).index().name(None);
        let mut table = builder.build();
        table.with(Style::modern());
        println!("{}", table);
    }
}

fn add_new_todo(mut todo_vec: Vec<String>, new_todo: &str) -> Vec<String> {
    let deserial_todo_vec: Vec<Todo> = todo_vec
        .iter()
        .map(|todo| serde_json::from_str(todo).expect("Failed to parse todo"))
        .collect();
    let id: u8;
    if !deserial_todo_vec.is_empty() {
        id = deserial_todo_vec.last().clone().unwrap().id + 1;
    } else {
        id = 1;
    }
    let new_todo_inst: Todo = Todo {
        id: id,
        status: "".to_string(),
        note: new_todo.to_string(),
    };
    let stringify_new_todo_inst = serde_json::to_string(&new_todo_inst);
    todo_vec.push(stringify_new_todo_inst.unwrap());
    todo_vec
}

fn clear_todo() {
    let confirm = utils::confirm_prompt(
        "Are you sure to clear the list ?",
        "It will clear completely the list.",
    );
    if !confirm {
        exit(0);
    }
    let app_data_path = utils::get_app_data();
    let mut projects = utils::get_app_vec();
    let get_current_workdir = utils::get_current_path();
    if let Some(pos) = projects
        .iter()
        .position(|app| app.location == get_current_workdir)
    {
        let app = projects.remove(pos);
        let mut updated_app = app;
        updated_app.todo = "[]".to_string();
        projects.push(updated_app);
        let update_data = projects::Data { project: projects };
        let save_json = serde_json::to_string(&update_data).expect("Failed to serialize data");
        fs::write(app_data_path, save_json).expect("Failed to write updated data");
    }
    logs::info_log("To-do list cleared successfully".to_string());
}

fn remove_todo() {
    let ask_id = utils::prompt_message(
        "Enter todo id you want to delete".to_string(),
        "Failed to get the user input".to_string(),
    );
    let app_data_path = utils::get_app_data();
    let mut projects = utils::get_app_vec();
    let get_current_workdir = utils::get_current_path();
    if let Some(pos) = projects
        .iter()
        .position(|app| app.location == get_current_workdir)
    {
        let app = projects.remove(pos);
        let mut updated_app = app.clone();
        let mut todo_vec = parse::parse_todo(app.todo.clone());
        let parse_todo_vec: Vec<Todo> = todo_vec
            .iter()
            .map(|todo| serde_json::from_str(todo).expect("Failed to parse todo"))
            .collect();
        // find the dodo and return it's index in the vector
        let find_todo_by_id = parse_todo_vec
            .iter()
            .position(|todo| todo.id == ask_id.parse::<u8>().unwrap())
            .unwrap();
        todo_vec.remove(find_todo_by_id);
        let stringify_todo_vec = parse::stringify_todo(todo_vec);
        updated_app.todo = stringify_todo_vec;
        projects.push(updated_app);
        let update_data = projects::Data { project: projects };
        let save_json = serde_json::to_string(&update_data).expect("Failed to serialize data");
        fs::write(app_data_path, save_json).expect("Failed to write updated data");
        logs::info_log("Successfully remove the to-do".to_string());
    }
}

//TODO
// can update one todo status by id
fn update_todo_status() {
    let app_data_path = utils::get_app_data();
    let mut projects = utils::get_app_vec();
    let get_current_workdir = utils::get_current_path();
    if let Some(pos) = projects
        .iter()
        .position(|app| app.location == get_current_workdir)
    {
        let app = projects.remove(pos);
        let mut updated_app = app.clone();
        let mut todo_vec = parse::parse_todo(app.todo.clone());
        let ask_todo =
            utils::get_select_option("Select the todo to update".to_string(), todo_vec.clone())
                .unwrap();
        let todo_status_vec: Vec<String> = vec_of_strings!("pending", "done", "wip");
        let mut updated_todo: Todo;
        let todo_to_update: Result<Todo, serde_json::Error> = serde_json::from_str(&ask_todo);
        match todo_to_update {
            Ok(todo) => {
                let select_status =
                    utils::get_select_option("Select the new status".to_string(), todo_status_vec)
                        .unwrap();
                // find the dodo and return it's index in the vectors
                let todo_vec_index = todo_vec.iter().position(|todo| todo == &ask_todo);
                todo_vec.remove(todo_vec_index.unwrap());
                updated_todo = todo;
                updated_todo.status = select_status;
                let stringify_updated_todo = serde_json::to_string(&updated_todo).unwrap();
                todo_vec.push(stringify_updated_todo);
            }
            Err(e) => {
                // Handle the error
                println!("Failed to parse todo: {:?}", e);
            }
        }
        let stringify_todo_vec = parse::stringify_todo(todo_vec);
        updated_app.todo = stringify_todo_vec;
        projects.push(updated_app);
        let update_data = projects::Data { project: projects };
        let save_json = serde_json::to_string(&update_data).expect("Failed to serialize data");
        fs::write(app_data_path, save_json).expect("Failed to write updated data");
        logs::info_log("Successfully update to-do status".to_string());
    }
}
