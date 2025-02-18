use crate::projects;
use crate::utils;

pub fn parse_todo_string() {
    let mut projects = utils::get_app_vec();
    let get_current_workdir = utils::get_current_path();
    let current_selected_project: projects::Project;
    if let Some(pos) = projects
        .iter()
        .position(|app| app.location == get_current_workdir)
    {
        let app = projects.remove(pos);
        current_selected_project = projects::Project {
            id: app.id,
            name: app.name,
            tech: app.tech,
            location: app.location,
            repository: app.repository,
            github_project: app.github_project,
            version: app.version,
            todo: app.todo,
        };
        // let updated_project = update_select_properties(current_selected_project, property);
        // projects.push(updated_project);
        // let update_data = Data { project: projects };
        // let save_json = serde_json::to_string(&update_data).expect("Failed to serialize data");
        // fs::write(app_data_path, save_json).expect("Failed to write updated data");
        println!("{:?}", current_selected_project);
        if current_selected_project.todo == "" {
            create_todo_struct();
        }
    }
}

fn create_todo_struct() {
    let new_todo: Vec<String> = Vec::new();
    let new_todo_string = serde_json::to_string(&new_todo).expect("Failed to serialize todo");
    new_todo_string
}

fn parse_todo(todo_string: &str) {
    let todo_decoded: Vec<String> =
        serde_json::from_str(&todo_string).expect("Failed to deserialize todo");
    todo_decoded
}
