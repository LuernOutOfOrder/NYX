/*!
This module provides various utility functions for the NYX project management tool.

# Functions

- `get_nyx_env_var`: Retrieves the NYX environment variable.
- `get_app_data`: Gets the path to the application data file.
- `get_app_vec`: Parses the application data file and returns a vector of `Project` structs.
- `get_app_vec_short`: Parses the application data file and returns a vector of `ProjectShort` structs.
- `get_current_path`: Returns the current working directory as a string.
- `get_render_config`: Returns a customized `RenderConfig` for the inquire library.
- `change_work_dir`: Changes the current working directory to the specified path.
- `get_tech_option`: Returns a vector of technology options.
- `get_select_app_option`: Prompts the user to select a technology option.
- `get_select_option`: Prompts the user to select an option from a given list.
- `get_project_property`: Returns a vector of project property names.
- `prompt_message`: Prompts the user with a message and returns the input.
- `nyx_ascii_art`: Returns the NYX ASCII art logo.
- `path_exists`: Checks if a given path exists.
- `rm_command`: Executes a command to remove a directory.
- `custom_throbber`: Returns a custom throbber with a message.
- `nyx_usage`: Returns the usage information for the NYX tool.
- `command_usage`: Prints the usage information and exits the program.
- `confirm_prompt`: Prompts the user with a confirmation message.

# Usage

This module is intended to be used internally by the NYX project management tool to handle various utility tasks such as environment variable retrieval, file operations, user prompts, and more.
*/

use crate::projects::nxp::NXPContent;
use crate::projects::{self};
use std::env::var;
use std::fs;
use std::io::Read;
use std::{
    env,
    fs::File,
    process::{exit, Command},
};

use inquire::{
    ui::{Attributes, Color, RenderConfig, StyleSheet, Styled},
    Confirm, InquireError, Select, Text,
};
use throbber::Throbber;

pub fn get_nyx_env_var() -> String {
    let env_var = "NYX";
    match env::var(env_var) {
        Ok(v) => return v,
        Err(e) => panic!("${} is not set ({})", env_var, e),
    }
}

pub fn get_app_data() -> String {
    let nyx_path = get_nyx_env_var();
    let app_data = nyx_path + "/src/data/app.json";
    return app_data;
}

pub fn get_app_vec() -> Vec<projects::Project> {
    let app_data_path = get_app_data();
    let json_data = fs::read_to_string(app_data_path.clone()).expect("Failed to read app data");
    let data: projects::Data = serde_json::from_str(&json_data).expect("Invalid JSON");
    let mut projects: Vec<projects::Project> = Vec::new();
    for app in &data.project {
        projects.push(projects::Project {
            id: app.id.clone(),
            name: app.name.clone(),
            tech: app.tech.clone(),
            location: app.location.clone(),
            repository: app.repository.clone(),
            github_project: app.github_project.clone(),
            version: app.version.clone(),
            todo: app.todo.clone(),
        });
    }
    return projects;
}

pub fn get_current_path() -> String {
    let path = env::current_dir().expect("Failed to get current directory");
    return path.display().to_string();
}

pub fn get_render_config() -> RenderConfig<'static> {
    let mut render_config = RenderConfig::default();
    render_config.prompt_prefix = Styled::new("?").with_fg(Color::LightMagenta);
    render_config.highlighted_option_prefix = Styled::new("➠").with_fg(Color::DarkMagenta);
    render_config.selected_checkbox = Styled::new("☑").with_fg(Color::LightMagenta);
    render_config.scroll_up_prefix = Styled::new("⇞").with_fg(Color::DarkMagenta);
    render_config.scroll_down_prefix = Styled::new("⇟").with_fg(Color::DarkMagenta);
    render_config.unselected_checkbox = Styled::new("☐").with_fg(Color::DarkMagenta);
    render_config.selected_option = Some(StyleSheet::new().with_fg(Color::DarkMagenta));
    render_config.text_input = StyleSheet::new().with_fg(Color::DarkMagenta);

    render_config.error_message = render_config
        .error_message
        .with_prefix(Styled::new("❌").with_fg(Color::LightRed));

    render_config.answer = StyleSheet::new()
        .with_attr(Attributes::ITALIC)
        .with_fg(Color::LightYellow);

    render_config.help_message = StyleSheet::new().with_fg(Color::DarkYellow);

    render_config
}

pub fn change_work_dir(dir: &String) {
    env::set_current_dir(&dir).expect("Failed to change directory");
}

pub fn get_tech_option() -> Vec<String> {
    let options: Vec<String> = vec![
        "Node.js".to_string(),
        "Python".to_string(),
        "Golang".to_string(),
        "Rust".to_string(),
        "C++".to_string(),
        "Other".to_string(),
    ];
    return options;
}

pub fn get_select_app_option(prompt: String) -> std::result::Result<String, InquireError> {
    let options = get_tech_option();

    let ans: std::result::Result<String, InquireError> = Select::new(&prompt, options).prompt();

    return ans;
}

pub fn get_select_option(
    prompt: String,
    option: Vec<String>,
) -> std::result::Result<String, InquireError> {
    let ans: std::result::Result<String, InquireError> = Select::new(&prompt, option).prompt();

    return ans;
}

pub fn prompt_message(message: String, error_message: String) -> String {
    inquire::set_global_render_config(get_render_config());
    let message = Text::new(&message).prompt().expect(&error_message);
    return message;
}

pub fn nyx_ascii_art() -> String {
    let ascii_art = r"         
 _                         
( (    /||\     /||\     /|
|  \  ( |( \   / )( \   / )
|   \ | | \ (_) /  \ (_) / 
| (\ \) |  \   /    ) _ (  
| | \   |   ) (    / ( ) \ 
| )  \  |   | |   ( /   \ )
|/    )_)   \_/   |/     \|

";

    return ascii_art.to_string();
}

pub fn path_exists(path: &str) -> bool {
    fs::metadata(path).is_ok()
}

pub fn rm_command(path: String) {
    Command::new("rm")
        .arg("-rf")
        .arg(path)
        .spawn()
        .expect("Failed to delete the directory of the project");
}

pub fn custom_throbber(message: String) -> Throbber {
    let custom_throbber = Throbber::new().message(message).frames(&throbber::ROTATE_F);
    return custom_throbber;
}

// pub fn unknown_flag(arg: &str) {
//     println!("Unknown flag parsed: {}", arg);
//     exit(1);
// }

pub fn nyx_usage() -> &'static str {
    let usage = r"
Usage: nyx command [options]

A simple tool to help manage your projects.

Commands:
    init            Initialize NYX data
    cat-nxs         Emit NXS object content
    cat-nxp         Emit specified NXP object content
    project         Initialize a new project
    project-add     Add an existing project to the projects list
    project-list    List all projects
    project-delete  Remove project from list or completely from storage.
    project-build   Build the current project in working directory
    project-update  Update specified project properties
    project-todo    Manage to-do list for current project
    cleanup         Cleanup all unused files
    git-stash       Stash with message
    git-tag         Create a new tag and push it to the origin branch
    git-reverse     Revert to the specified commit
    git-summarize   Show a summary of the current Git repository
    health          Display current development system health
    update          Update the current version of NYX
    help            Show this help message

Options:

    -h, --help      Show command usage
    -v, --version   Show the current version of NYX
";

    return usage;
}

pub fn command_usage(usage: &str) {
    println!("{}", usage);
    exit(0);
}

pub fn confirm_prompt(message: &str, help_message: &str) -> bool {
    let ans = Confirm::new(message)
        .with_default(false)
        .with_help_message(help_message)
        .prompt();
    ans.unwrap()
}

/// The `update_editor` function updates a file using the specified editor and prints its
/// content.
///
/// Arguments:
///
/// * `buffer`: The `buffer` parameter in the `update_editor` function is a vector of unsigned 8-bit
/// integers (`Vec<u8>`) that represents the content to be written to a temporary file and opened in the
/// user's preferred text editor for editing.
pub fn update_editor(content: NXPContent) -> Vec<u8> {
    change_work_dir(&get_nyx_env_var());
    let editor = match var("EDITOR") {
        Ok(str) => str,
        Err(e) => {
            lrncore::logs::info_log(&format!("EDITOR var not defined: {}", e));
            "vim".to_string()
        }
    };
    let file_path = ".data/tmp/PROJECT_EDIT";
    match File::create(&file_path) {
        Ok(f) => f,
        Err(e) => {
            lrncore::logs::error_log(&format!("Failed to temp file: {}", e));
            return vec![];
        }
    };
    let json = match serde_json::to_string_pretty(&content) {
        Ok(str) => str,
        Err(e) => {
            lrncore::logs::error_log(&format!("Failed to parse project content to JSON: {}", e));
            return vec![];
        }
    };
    match fs::write(file_path, json) {
        Ok(_) => (),
        Err(e) => {
            lrncore::logs::error_log(&format!(
                "Failed to write current project buffer to temp file: {}",
                e
            ));
            return vec![];
        }
    }
    Command::new(editor)
        .arg(&file_path)
        .status()
        .expect("Something went wrong");

    let mut editable = String::new();
    match File::open(file_path)
        .expect("Could not open file")
        .read_to_string(&mut editable)
    {
        Ok(_) => (),
        Err(e) => {
            lrncore::logs::error_log(&format!("Failed to open temp file: {}", e));
            return vec![];
        }
    }
    let update_content: NXPContent = match serde_json::from_str(&editable) {
        Ok(content) => content,
        Err(e) => {
            lrncore::logs::error_log(&format!("Failed to write JSON str to struct: {}", e));
            return vec![];
        }
    };
    let buffer: Vec<u8> =
        bincode::serialize(&update_content).expect("Failed to serialize updated content struct");
    return buffer;
}
