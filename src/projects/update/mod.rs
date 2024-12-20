use inquire::Text;

use crate::utils;

pub fn update_project() {
    let projects = utils::get_app_vec();
    inquire::set_global_render_config(utils::get_render_config());
    let app_name = Text::new("Enter the name of the project:")
        .prompt()
        .expect("Failed to read project name");
    // if an index match the given data,
    if let Some(pos) = projects.iter().position(|app| app.name == app_name) {
        let project = projects.get(pos);
        print!("{:?}", project);
    }
}

// fn update_by_tech(tech: &str) {
//     match tech {
//       tech if tech
//     }
// }
