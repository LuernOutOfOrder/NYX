use crate::nxfs::config::LogLevel;
use crate::projects::fs::File;
use crate::todo::Todo;
use crate::todo::{TodoContent, TodoFile, TodoHeader};
use crate::todo::{FORMAT_VERSION, MAGIC_NUMBER};

use crate::todo::exit;
use crate::todo::BufReader;
use crate::utils::log;
use std::io::Read;
use std::io::Write;

use lrncore::vec_of_strings;

pub fn add_new_todo(mut todo_vec: Vec<Todo>, new_todo: &str) -> Vec<Todo> {
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

pub fn create_todo_file(hash: &str) {
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
            log::log_from_log_level(
                LogLevel::Error,
                &format!("Failed to create todo file: {}", e),
            );
            return;
        }
    };
    match todo_file.write_all(&file_buff) {
        Ok(_) => (),
        Err(e) => {
            log::log_from_log_level(
                LogLevel::Error,
                &format!("Failed to write buffer in todo file: {}", e),
            );
        }
    }
    log::log_from_log_level(LogLevel::Info, "Create new todo file");
}

pub fn parse_todo_file(hash: &str) -> TodoFile {
    let file = match File::open(format!(".nxfs/projects/{}/todo", hash)) {
        Ok(f) => f,
        Err(e) => {
            log::log_from_log_level(LogLevel::Error, &format!("Failed to open todo file: {}", e));
            exit(51);
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
                log::log_from_log_level(LogLevel::Error, &format!("Failed to read byte: {}", e));
                exit(50)
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

pub fn update_todo_file(hash: &str, vec: Vec<Todo>, todo: TodoFile) {
    let file_path = format!(".nxfs/projects/{}/todo", hash);
    let update_todo: TodoFile = TodoFile {
        header: todo.header,
        content: TodoContent { todos: vec },
    };
    let buff: Vec<u8> =
        bincode::serialize(&update_todo).expect("Failed to serialize updated todo file");
    let mut todo_file: File = match File::create(file_path) {
        Ok(f) => f,
        Err(e) => {
            log::log_from_log_level(
                LogLevel::Error,
                &format!("Failed to update todo file: {}", e),
            );
            return;
        }
    };
    match todo_file.write_all(&buff) {
        Ok(_) => (),
        Err(e) => {
            log::log_from_log_level(
                LogLevel::Error,
                &format!("Failed to write buffer in todo file: {}", e),
            );
            return;
        }
    }
    log::log_from_log_level(LogLevel::Info, "Successfully update todo file");
}

pub fn todos_status_list() -> Vec<String> {
    let vec = vec_of_strings!("pending", "done", "wip");
    vec
}
