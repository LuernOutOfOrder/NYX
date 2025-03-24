use inquire::{InquireError, Select};
use serde::{Deserialize, Serialize};
use std::env;
use std::io::BufReader;
use std::io::Read;
use std::io::Write;
use std::path::Path;
use std::process::exit;
use tabled::Table;
use tabled::{settings::Style, Tabled};

mod parse;

use crate::projects::nxs;
use crate::{logs, projects, utils, vec_of_strings};
use std::fs::{self, File};

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

#[derive(Debug, Deserialize, Serialize)]
pub struct TodoContent {
    pub todos: Vec<Todo>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
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
    let args: Vec<String> = env::args().collect();
    if let Some(arg) = args.iter().last() {
        //todo
        // flag to list or add directly to the todo
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
                show_todo();
                exit(0);
            }
            "--show" => {
                show_todo();
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
        choice if choice == "Show to-do list" => show_todo(),
        choice if choice == "Clear the to-do list" => prune_todo(),
        choice if choice == "Remove one to-do" => remove_todo(),
        choice if choice == "Update one to-do status" => update_todo_status(),
        _ => println!("please make a choice"),
    }
}

fn update_todo_list() {
    utils::change_work_dir(&utils::get_nyx_env_var());
    let new_todo = utils::prompt_message(
        "Enter new todo:".to_string(),
        "Error getting user input".to_string(),
    );
    let mut projects = nxs::get_all_project();
    let app_name = utils::prompt_message(
        "Enter project name:".to_string(),
        "Error with the project name referred".to_string(),
    );
    let mut project_name: String = String::new();
    let mut project_hash: [u8; 11] = [0u8; 11];
    if let Some(pos) = projects.iter().position(|app| app.project_name == app_name) {
        let app = projects.remove(pos);
        project_name = app.project_name;
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
    println!("todo {:?}", todo);
    // let current_todo_vec = nxp.content.todo;
    // let new_todo_vec = add_new_todo(current_todo_vec, &new_todo);
    // nxp.content.todo = new_todo_vec;
    // let mut buffer = bincode::serialize(&nxp).expect("Failed to serialize NXP structure");
}

fn show_todo() {
    // let mut projects = utils::get_app_vec();
    // let get_current_workdir = utils::get_current_path();
    // if let Some(pos) = projects
    //     .iter()
    //     .position(|app| app.location == get_current_workdir)
    // {
    //     let app = projects.remove(pos);
    //     let todo_vec = parse::parse_todo(app.todo.clone());
    //     let parse_todo_vec: Vec<Todo> = todo_vec
    //         .iter()
    //         .map(|todo| serde_json::from_str(todo).expect("Failed to parse todo"))
    //         .collect();
    //     let builder = Table::builder(&parse_todo_vec).index().name(None);
    //     let mut table = builder.build();
    //     table.with(Style::modern());
    //     println!("{}", table);
    // }
}

fn add_new_todo(mut todo_vec: Vec<Todo>, new_todo: &str) -> Vec<Todo> {
    let deserial_todo_vec: Vec<Todo> = todo_vec.clone();
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
    todo_vec.push(new_todo_inst);
    todo_vec
}

fn prune_todo() {
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

fn create_todo_file(hash: &str) {
    utils::change_work_dir(&utils::get_nyx_env_var());
    let content: TodoContent = TodoContent { todos: vec![] };
    let content_buff: Vec<u8> =
        bincode::serialize(&content).expect("Failed to serialize todo content");
    // header
    let header: TodoHeader = TodoHeader {
        magic_number: MAGIC_NUMBER,
        format_version: FORMAT_VERSION,
        project_id: {
            let mut arr = [0u8; 11];
            let bytes = hash.as_bytes();
            arr[..bytes.len()].copy_from_slice(bytes);
            arr
        },
        project_size: content_buff.len() as u32,
        reserved: 0,
    };
    let header_buff: Vec<u8> =
        bincode::serialize(&header).expect("Failed to serialize todo header buffer");
    // file
    let mut file_buff: Vec<u8> = Vec::new();
    file_buff.extend_from_slice(&header_buff);
    file_buff.extend_from_slice(&content_buff);
    let file_path = format!(".nxfs/projects/{}/todo", hash);
    let mut todo_file: File = match File::create(file_path) {
        Ok(f) => f,
        Err(e) => {
            lrncore::logs::error_log(&format!("Failed to create todo file: {}", e));
            return;
        }
    };
    match todo_file.write_all(&file_buff) {
        Ok(_) => (),
        Err(e) => {
            lrncore::logs::error_log(&format!("Failed to write buffer in todo file: {}", e));
        }
    }
    lrncore::logs::info_log("Create new todo file");
}

fn parse_todo_file(hash: &str) -> TodoFile {
    utils::change_work_dir(&utils::get_nyx_env_var());
    let file = match File::open(format!(".nxfs/projects/{}/todo", hash)) {
        Ok(f) => f,
        Err(e) => {
            lrncore::logs::error_log(&format!("Failed to open todo file: {}", e));
            exit(1);
        }
    };
    // initialize TodoHeader size from structure and buffer
    let header_size = std::mem::size_of::<TodoHeader>();
    let buffer = BufReader::new(file);
    // vector containing the whole todo file
    let mut bytes_vec: Vec<u8> = Vec::new();
    for byte_or_error in buffer.bytes() {
        match byte_or_error {
            Ok(byte) => bytes_vec.push(byte),
            Err(e) => {
                lrncore::logs::error_log(&format!("Failed to read byte: {}", e));
                exit(1)
            }
        }
    }

    let header_bytes = &bytes_vec[..header_size];
    // convert into the TodoHeader struct
    let header: TodoHeader =
        bincode::deserialize(header_bytes).expect("Failed to deserialize TodoHeader");
    // content
    let todo_content_bytes = &bytes_vec[header_size..];
    let todo_content: TodoContent =
        bincode::deserialize(todo_content_bytes).expect("Failed to deserialize todo content");
    let todo: TodoFile = TodoFile {
        header: header,
        content: todo_content,
    };
    todo
}
