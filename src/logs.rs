use colored::Colorize;

pub fn info_log(msg: String) {
    let info = "[INFO]".truecolor(0, 255, 0);
    println!("{} {}", info, msg);
}

pub fn error_log(msg: String) {
    let info = "[ERROR]".truecolor(255, 0, 0);
    println!("{} {}", info, msg);
}

pub fn error_log_with_code(msg: String, error: String) {
    let info = "[ERROR]".truecolor(255, 0, 0);
    println!("{} {} {}", info, msg, error);
}

pub fn active_log(msg: &str) {
    let active = "active".truecolor(0, 255, 0);
    println!("{} {}", msg, active);
}

pub fn inactive_log(msg: &str) {
    let inactive = "inactive".truecolor(255, 0, 0);
    println!("{} {}", msg, inactive);
}
