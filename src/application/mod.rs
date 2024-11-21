use inquire::{InquireError, Select};
use std::process::Command;

pub fn new_nodejs_app(name: String) {
    let options: Vec<&str> = vec!["Node.js", "Python", "Golang", "Rust", "C++"];

    let ans: Result<&str, InquireError> =
        Select::new("Which tech do you want to use ?", options).prompt();

    match ans {
        Ok(choice) => println!("{}!", choice),
        Err(_) => println!("There was an error, please try again"),
    }
    match Command::new("mkdir").arg(name).spawn() {
        Ok(_) => println!("Directory created successfully"),
        Err(e) => println!("Failed to create directory: {}", e),
    }
}
