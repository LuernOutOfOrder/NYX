use inquire::{InquireError, Select};
use std::env;
use std::path::Path;
use std::process::Command;

pub fn new_nodejs_app(name: String) {
    let options: Vec<&str> = vec!["Node.js", "Python", "Golang", "Rust", "C++"];

    let ans: Result<&str, InquireError> =
        Select::new("Which tech do you want to use ?", options).prompt();

    match ans {
        Ok(choice) => println!("{}!", choice),
        Err(_) => println!("There was an error, please try again"),
    }
    match Command::new("mkdir").arg(name.clone()).spawn() {
        Ok(_) => println!("Successfully created the new application directory"),
        Err(e) => println!("Failed to create directory: {}", e),
    }
    change_work_dir(name.clone());
    // match Command::new("mkdir").arg(name).spawn() {
    //     Ok(_) => println!("Directory created successfully"),
    //     Err(e) => println!("Failed to create directory: {}", e),
    // }
}

fn change_work_dir(dir: String) {
    let new_app_path = format!("/{}\n", dir);
    print!("{}", new_app_path);
    env::set_current_dir(&new_app_path).expect("Failed to change directory");
}
