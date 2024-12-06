use colored::Colorize;

pub fn update_bin() {
    let nyx_ascii_art = r"         
 _                         
( (    /||\     /||\     /|
|  \  ( |( \   / )( \   / )
|   \ | | \ (_) /  \ (_) / 
| (\ \) |  \   /    ) _ (  
| | \   |   ) (    / ( ) \ 
| )  \  |   | |   ( /   \ )
|/    )_)   \_/   |/     \|

"
    .truecolor(138, 43, 226);
    println!("{}", nyx_ascii_art);
}
