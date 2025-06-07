use std::process::Command;

pub fn folder_size(path: &str) -> String {
    let du = Command::new("du")
        .arg("-sh")
        .arg(path)
        .output()
        .expect("Failed to execute du command");
    str::from_utf8(&du.stdout).unwrap().to_string()
}
