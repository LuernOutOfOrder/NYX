use inquire::{InquireError, Select};
use std::env;
use std::fs;
use std::path::PathBuf;

pub fn new_nodejs_app(name: String) {
    let options: Vec<&str> = vec!["Node.js", "Python", "Golang", "Rust", "C++"];

    let ans: Result<&str, InquireError> =
        Select::new("Which tech do you want to use ?", options).prompt();

    match ans {
        Ok(choice) => println!("{}!", choice),
        Err(_) => println!("There was an error, please try again"),
    }
    match fs::create_dir(name.clone()) {
        Ok(_) => println!("Directory created successfully"),
        Err(e) => println!("Failed to create directory: {}", e),
    }
    change_work_dir(&name);
}

fn change_work_dir(dir: &String) {
    env::set_current_dir(&dir).expect("Failed to change directory");
}
