/*!
This module provides various utility functions for the NYX project management tool.

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

pub fn get_select_project_option(prompt: String) -> std::result::Result<String, InquireError> {
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

pub fn confirm_prompt(message: &str, help_message: &str) -> bool {
    let ans = Confirm::new(message)
        .with_default(false)
        .with_help_message(help_message)
        .prompt();
    ans.unwrap()
}
