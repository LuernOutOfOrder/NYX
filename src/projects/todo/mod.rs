use inquire::{InquireError, Select};
use serde::{Deserialize, Serialize};
use std::env;
use std::io::BufReader;
use std::path::Path;
use std::process::exit;
use tabled::Table;
use tabled::{settings::Style, Tabled};

mod helpers;

use helpers::{
    add_new_todo, create_todo_file, parse_todo_file, todos_status_list, update_todo_file,
};
use lrncore::usage_exit::command_usage;

use crate::projects::nxs;
use crate::{logs, utils};
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

    usage.to_string()
}

pub fn choose_todo() {
    lrncore::path::change_work_dir(&utils::env::get_nyx_env_var());
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
                command_usage(&todo_help());
            }
            "--help" => {
                command_usage(&todo_help());
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
        "Add to to-do" => update_todo_list(),
        "Show to-do list" => display_todo_list(),
        "Clear the to-do list" => prune_todo(),
        "Remove one to-do" => remove_todo(),
        "Update one to-do status" => update_todo_status(),
        _ => println!("please make a choice"),
    }
}

fn update_todo_list() {
    let app_name = utils::prompt_message(
        "Enter project name:".to_string(),
        "Error with the project name referred".to_string(),
    );
    let new_todo = utils::prompt_message(
        "Enter new todo:".to_string(),
        "Error getting user input".to_string(),
    );
    let mut projects = nxs::get_all_project();
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
    let project_name = utils::prompt_message(
        "Enter project name:".to_string(),
        "Error with the user input".to_string(),
    );
    #[allow(unused_assignments)]
    let mut project_hash: [u8; 11] = [0u8; 11];
    if let Some(pos) = projects
        .iter()
        .position(|app| app.project_name == project_name)
    {
        let app = projects.remove(pos);
        project_hash = app.project_hash
    } else {
        lrncore::logs::error_log("Project not found");
        exit(1);
    }
    let project_hash_str = String::from_utf8_lossy(&project_hash);

    let confirm = utils::prompt::confirm_prompt("Are you sure you want to prune the todo list ?", "You cannot undo the changes");
    if !confirm {
        return;
    }

    let file_path = format!(".nxfs/projects/{}/todo", project_hash_str);
    match fs::remove_file(file_path) {
        Ok(_) => {
            lrncore::logs::time_info_log("Successfully remove todo file");
        }
        Err(e) => {
            lrncore::logs::error_log(&format!("Failed to remove todo file: {}", e));
        }
    };
    logs::info_log("To-do list cleared successfully".to_string());
}

fn remove_todo() {
    let mut projects = nxs::get_all_project();
    let project_name = utils::prompt_message(
        "Enter project name:".to_string(),
        "Error getting user input".to_string(),
    );
    let ask_id = utils::prompt_message(
        "Enter todo id you want to delete:".to_string(),
        "Failed to get the user input".to_string(),
    );
    let project_hash: [u8; 11];
    if let Some(pos) = projects
        .iter()
        .position(|app| app.project_name == project_name)
    {
        let app = projects.remove(pos);
        project_hash = app.project_hash;
    } else {
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
    update_todo_file(&project_hash_str, todo_vec, todo);
    logs::info_log("Successfully remove the to-do".to_string());
}

fn update_todo_status() {
    let project_name = utils::prompt_message(
        "Enter project name:".to_string(),
        "Error getting user input".to_string(),
    );
    let ask_todo_id = utils::prompt_message(
        "Enter todo id:".to_string(),
        "Error getting user input".to_string(),
    );
    let new_status =
        utils::get_select_option("Select new status:".to_string(), todos_status_list());
    let mut projects = nxs::get_all_project();
    let mut project_hash: [u8; 11] = [0u8; 11];
    if let Some(pos) = projects
        .iter()
        .position(|app| app.project_name == project_name)
    {
        let app = projects.remove(pos);
        project_hash = app.project_hash;
    }
    let project_hash_str = String::from_utf8_lossy(&project_hash);
    let todo: TodoFile = parse_todo_file(&project_hash_str);
    let mut todo_vec: Vec<Todo> = todo.content.todos.clone();
    let todo_id_stdi = ask_todo_id
        .parse::<u8>()
        .expect("Failed to parse todo id to u8");
    let mut updated_todo: Todo = Todo {
        note: String::new(),
        status: String::new(),
        id: 0,
    };
    if let Some(todo) = todo_vec.iter().position(|todo| todo.id == todo_id_stdi) {
        let find_todo = todo_vec.remove(todo);
        updated_todo.id = find_todo.id;
        updated_todo.note = find_todo.note;
    };
    let todo: TodoFile = parse_todo_file(&project_hash_str);
    updated_todo.status = new_status.expect("Failed to get the updated status");
    todo_vec.push(updated_todo);
    update_todo_file(&project_hash_str, todo_vec, todo);
    logs::info_log("Successfully update to-do status".to_string());
}
