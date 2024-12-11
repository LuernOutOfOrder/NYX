use std::env;

use inquire::ui::{Attributes, Color, RenderConfig, StyleSheet, Styled};

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
    ];
    return options;
}
