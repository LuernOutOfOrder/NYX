use colored::Colorize;

pub fn not_installed(msg: &str) {
    let not_installed = "not installed".truecolor(255, 0, 0);
    println!("\t{} {}", msg, not_installed);
}

pub fn installed(msg: &str) {
    let installed = "installed".truecolor(0, 255, 0);
    println!("\t{} {}", msg, installed);
}
