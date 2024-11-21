use inquire::{Text}

pub fn new_nodejs_app(name: String, types: String) {
    let name = Text::new("How du you want to name your app?").prompt();

    match name {
        Ok(name) => println!("Hello {}", name),
        Err(_) => {
            println!("An error happened when asking for the application name, try again later.")
        }
    }
}
