use std::process::Command;

pub fn folder_size(path: &str) -> String {
    let du = Command::new("du").arg("-sh").arg(path).output().expect("Failed to execute du command");
    format!("{}", String::from_utf8_lossy(&du.stdout))
}
