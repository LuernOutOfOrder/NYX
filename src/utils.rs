use crate::projects::{self};
use std::{env, fs, process::Command};

use inquire::{
    ui::{Attributes, Color, RenderConfig, StyleSheet, Styled},
    InquireError, Select, Text,
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

pub fn get_project_property() -> Vec<String> {
    let options: Vec<String> = vec![
        "id".to_string(),
        "name".to_string(),
        "tech".to_string(),
        "location".to_string(),
    ];
    return options;
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
