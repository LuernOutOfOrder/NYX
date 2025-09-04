/*!
This module provides various utility functions for the NYX project management tool.

This module is intended to be used internally by the NYX project management tool to handle various utility tasks such as environment variable retrieval, file operations, user prompts, and more.
*/

use crate::nxfs::config::parse_config_file;
use crate::nxfs::nxp::NXPContent;
use std::fs::write;
use std::{
    fs::File,
    process::{exit, Command},
};

use inquire::{
    ui::{Attributes, Color, RenderConfig, StyleSheet, Styled},
    InquireError, Select, Text,
};
use throbber::Throbber;
pub mod editor;
pub mod env;
pub mod fsys;
pub mod log;
pub mod prompt;

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
    let config = parse_config_file().expect("Failed to parse config file");
    config.plugins.list
}

pub fn get_select_project_option(prompt: &str) -> std::result::Result<String, InquireError> {
    let options = get_tech_option().into_iter().map(|s| s.to_owned()).collect();

    let ans: std::result::Result<String, InquireError> = Select::new(prompt, options).prompt();

    ans
}

pub fn get_select_option(
    prompt: &str,
    option: Vec<&str>,
) -> std::result::Result<String, InquireError> {
    let options: Vec<String> = option.into_iter().map(|s| s.to_string()).collect();
    let ans: std::result::Result<String, InquireError> = Select::new(prompt, options).prompt();

    ans
}

pub fn prompt_message(message: &str, error_message: &str) -> String {
    inquire::set_global_render_config(get_render_config());
    let message = Text::new(message).prompt().expect(error_message);
    message.to_lowercase()
}

pub fn nyx_ascii_art() -> &'static str{
    (r"         
 _                         
( (    /||\     /||\     /|
|  \  ( |( \   / )( \   / )
|   \ | | \ (_) /  \ (_) / 
| (\ \) |  \   /    ) _ (  
| | \   |   ) (    / ( ) \ 
| )  \  |   | |   ( /   \ )
|/    )_)   \_/   |/     \|

") as _
}

pub fn custom_throbber(message: &str) -> Throbber {
    Throbber::new().message(message.to_owned()).frames(&throbber::ROTATE_F)
}
