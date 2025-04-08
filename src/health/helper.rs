use colored::Colorize;

pub fn not_installed(msg: &str) {
    let not_installed = "[✘]".truecolor(255, 0, 0);
    println!("\t{} {}", not_installed, msg);
}

pub fn installed(msg: &str) {
    let installed = "[✔]".truecolor(0, 255, 0);
    println!("\t{} {}", installed, msg);
}

pub fn warning(msg: &str) {
    let warning = "[⚠]".truecolor(255, 155, 0);
    println!("\t{} {}", warning, msg);
}
