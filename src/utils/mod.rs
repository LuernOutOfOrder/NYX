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
use std::env::var;
use std::fs::write;
use std::{
    fs::File,
    process::{exit, Command},
};

use inquire::{
    ui::{Attributes, Color, RenderConfig, StyleSheet, Styled},
    Confirm, InquireError, Select, Text,
};
use throbber::Throbber;
pub mod editor;
pub mod env;
pub mod fsys;

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

pub fn custom_throbber(message: String) -> Throbber {
    let custom_throbber = Throbber::new().message(message).frames(&throbber::ROTATE_F);
    return custom_throbber;
}

pub fn nyx_usage() -> &'static str {
    let usage = r"
Usage: nyx command [options]

A lightweight utility for efficient project management and useful tools.

Commands:
    init            Initialize NYX data
    cat-nxs         Emit NXS object content
    cat-nxp         Emit specified NXP object content
    project         Manage project-related tasks
    cleanup         Cleanup all unused files
    git             Git command wrapped in a simplified interface
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
