use inquire::{InquireError, Select};
use serde::{Deserialize, Serialize};
use std::env;
use std::io::BufReader;
use std::path::Path;
use std::process::exit;
use tabled::Table;
use tabled::{settings::Style, Tabled};

mod helpers;
mod parse;

use helpers::{add_new_todo, create_todo_file, parse_todo_file, update_todo_file};

use crate::projects::nxs;
use crate::{logs, projects, utils, vec_of_strings};
use std::fs::{self};

#[derive(Debug, Deserialize, Serialize)]
pub struct TodoFile {
    header: TodoHeader,
    content: TodoContent,
}

#[derive(Debug, Deserialize, Serialize)]
#[repr(C, packed)]
pub struct TodoHeader {
    pub magic_number: [u8; 8],
    pub format_version: [u8; 6],
    pub project_id: [u8; 11],
    pub project_size: u32,
    pub reserved: u32,
}

const MAGIC_NUMBER: [u8; 8] = *b"NXPTODO\0";
const FORMAT_VERSION: [u8; 6] = *b"0.1.0\0";

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct TodoContent {
    pub todos: Vec<Todo>,
}

#[derive(Debug, Deserialize, Serialize, Clone, Tabled)]
pub struct Todo {
    pub note: String,
    pub status: String,
    pub id: u8,
}

pub fn todo_help() -> String {
    let usage = r"
Usage: nyx project-todo

Options:

    -a, --add       Add new to-do to the list
    -s, --show      List all todo
    -p, --prune     Clear the todo list
    -r, --remove    Remove one todo by id
    -u, --update    Update one todo
    -h, --help      Show this help message
";

    return usage.to_string();
}

pub fn choose_todo() {
    utils::change_work_dir(&utils::get_nyx_env_var());
    let args: Vec<String> = env::args().collect();
    if let Some(arg) = args.iter().last() {
        match arg.as_str().trim() {
            "-a" => {
                update_todo_list();
                exit(0);
            }
            "--add" => {
                update_todo_list();
                exit(0);
            }
            "-s" => {
                display_todo_list();
                exit(0);
            }
            "--show" => {
                display_todo_list();
                exit(0);
            }
            "-p" => {
                prune_todo();
                exit(0);
            }
            "--prune" => {
                prune_todo();
                exit(0);
            }
            "-r" => {
                remove_todo();
                exit(0);
            }
            "--remove" => {
                remove_todo();
                exit(0);
            }
            "-u" => {
                update_todo_status();
                exit(0);
            }
            "--update" => {
                update_todo_status();
                exit(0);
            }

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
        choice if choice == "Show to-do list" => display_todo_list(),
        choice if choice == "Clear the to-do list" => prune_todo(),
        choice if choice == "Remove one to-do" => remove_todo(),
        choice if choice == "Update one to-do status" => update_todo_status(),
        _ => println!("please make a choice"),
    }
}

fn update_todo_list() {
    let new_todo = utils::prompt_message(
        "Enter new todo:".to_string(),
        "Error getting user input".to_string(),
    );
    let mut projects = nxs::get_all_project();
    let app_name = utils::prompt_message(
        "Enter project name:".to_string(),
        "Error with the project name referred".to_string(),
    );
    #[allow(unused_assignments)]
    let mut project_hash: [u8; 11] = [0u8; 11];
    if let Some(pos) = projects.iter().position(|app| app.project_name == app_name) {
        let app = projects.remove(pos);
        project_hash = app.project_hash
    } else {
        lrncore::logs::error_log("Project not found");
        exit(1);
    }

    let project_hash_str = String::from_utf8_lossy(&project_hash);
    let todo_file = format!(".nxfs/projects/{}/todo", project_hash_str);
    if !Path::new(&todo_file).exists() {
        create_todo_file(&project_hash_str);
    }
    let todo: TodoFile = parse_todo_file(&project_hash_str);
    let todo_vec: Vec<Todo> = todo.content.todos.clone();
    let todo_vec_update = add_new_todo(todo_vec, &new_todo);
    update_todo_file(&project_hash_str, todo_vec_update, todo);
}

fn display_todo_list() {
    // get todos from project name
    let mut projects = nxs::get_all_project();
    let app_name = utils::prompt_message(
        "Enter project name:".to_string(),
        "Error with the project name referred".to_string(),
    );
    #[allow(unused_assignments)]
    let mut project_hash: [u8; 11] = [0u8; 11];
    if let Some(pos) = projects.iter().position(|app| app.project_name == app_name) {
        let app = projects.remove(pos);
        project_hash = app.project_hash
    } else {
        lrncore::logs::error_log("Project not found");
        exit(1);
    }
    let project_hash_str = String::from_utf8_lossy(&project_hash);
    let todo: TodoFile = parse_todo_file(&project_hash_str);
    let todo_vec: Vec<Todo> = todo.content.todos.clone();
    // display todos
    let builder = Table::builder(&todo_vec).index().name(None);
    let mut table = builder.build();
    table.with(Style::modern());
    println!("{}", table);
}

fn prune_todo() {
    let mut projects = nxs::get_all_project();
    let project_name = utils::prompt_message("Enter project name:".to_string(), "Error with the user input".to_string());
 #[allow(unused_assignments)]
    let mut project_hash: [u8; 11] = [0u8; 11];
    if let Some(pos) = projects.iter().position(|app| app.project_name == project_name) {
        let app = projects.remove(pos);
        project_hash = app.project_hash
    } else {
        lrncore::logs::error_log("Project not found");
        exit(1);
    }
    let project_hash_str = String::from_utf8_lossy(&project_hash);
    
    let confirm = utils::confirm_prompt(
        "Are you sure to clear the list ?",
        "It will remove completely the todo file from your system.",
    );
    if !confirm {
        exit(0);
    }
    let file_path = format!(".nxfs/projects/{}/todo", project_hash_str);
    match fs::remove_file(file_path) {
        Ok(_) => {
            lrncore::logs::time_info_log("Successfully remove todo file");
        },
        Err(e) => {
            lrncore::logs::error_log(&format!("Failed to remove todo file: {}", e));
        }
    };
    logs::info_log("To-do list cleared successfully".to_string());
}

fn remove_todo() {
    let mut projects = nxs::get_all_project();
    let project_name = utils::prompt_message("Enter project name:".to_string(), "Error getting user input".to_string());
    let ask_id = utils::prompt_message(
        "Enter todo id you want to delete:".to_string(),
        "Failed to get the user input".to_string(),
    );
    let project_hash: [u8;11];
   if let Some(pos) = projects
        .iter()
        .position(|app| app.project_name == project_name)
    {
        let app = projects.remove(pos);
        project_hash = app.project_hash;
    }else {
        lrncore::logs::error_log("Project not found");
        exit(1);
    }

    let project_hash_str = String::from_utf8_lossy(&project_hash);
    let todo: TodoFile = parse_todo_file(&project_hash_str);
    let mut todo_vec: Vec<Todo> = todo.content.todos.clone();
    let todo_id_stdi = ask_id.parse::<u8>().expect("Failed to parse todo id to u8");
    if let Some(todo) = todo_vec.iter().position(|todo| todo.id == todo_id_stdi) {
        todo_vec.remove(todo);
    };
   update_todo_file(&project_hash_str, todo_vec, todo);logs::info_log("Successfully remove the to-do".to_string());
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
