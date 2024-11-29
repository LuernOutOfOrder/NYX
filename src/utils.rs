use std::env;

pub fn get_nyx_env_var() -> String {
    let env_var = "NYX";
    match env::var(env_var) {
        Ok(v) => return v,
        Err(e) => panic!("${} is not set ({})", env_var, e),
    }
}

pub fn get_current_path() -> String {
    let path = env::current_dir().expect("Failed to get current directory");
    return path.display().to_string();
}
