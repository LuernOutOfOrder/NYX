use lrncore::path::change_work_dir;
use toml::de::Error as toml_error;

use crate::utils;

use super::Plugin;

/// Parse given plugin and return struct
pub fn parse_plugin_file(name: &str) -> Result<Plugin, toml_error> {
    change_work_dir(&utils::env::get_nyx_env_var());
    let file_path: &str = &format!("plugins/{name}.toml");
let file =
        std::fs::read_to_string(file_path).expect("Failed to read the plugin file to string");
    let plugin: Plugin = match toml::from_str(&file) {
        Ok(p) => p,
        Err(e) => {
            println!("Failed to parse the configuration file: {e:?}");
            return Err(e);
        }
    };
    Ok(plugin)
}
