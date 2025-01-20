use regex::Regex;
use std::{env, fs};

use crate::{
    projects::{self, Data, Project},
    utils,
};

pub fn project_update_help() -> String {
    let usage = r"
Usage: nyx project-update

Options:

    -h, --help      Show this help message
";

    return usage.to_string();
}

pub fn update_project_properties() {
    let args: Vec<String> = env::args().collect();
    if let Some(arg) = args.iter().last() {
        match arg.as_str().trim() {
            "-h" => {
                utils::command_usage(&project_update_help());
            }
            "--help" => {
                utils::command_usage(&project_update_help());
            }
            _ => {}
        }
    }
    let app_data_path = utils::get_app_data();
    let mut projects = utils::get_app_vec();
    inquire::set_global_render_config(utils::get_render_config());
    let app_id = utils::prompt_message(
        "Enter project id".to_string(),
        "Error with the project id referred".to_string(),
    );
    // if an index match the given data,
    let property = utils::get_select_option(
        "Which property do you want to update ?".to_string(),
        utils::get_project_property(),
    )
    .expect("Failed to get select option");
    let current_selected_project: projects::Project;
    if let Some(pos) = projects.iter().position(|app| app.id == app_id) {
        println!("Project found");
        let app = projects.remove(pos);
        current_selected_project = Project {
            id: app.id,
            name: app.name,
            tech: app.tech,
            location: app.location,
            repository: app.repository,
            github_project: app.github_project,
            version: app.version,
        };
        let updated_project = update_select_properties(current_selected_project, property);
        projects.push(updated_project);
        let update_data = Data { project: projects };
        let save_json = serde_json::to_string(&update_data).expect("Failed to serialize data");
        fs::write(app_data_path, save_json).expect("Failed to write updated data");
    }
}

fn update_select_properties(project: projects::Project, property: String) -> projects::Project {
    match property {
        property if property == "id" => update_project_id(project),
        property if property == "name" => update_project_name(project),
        property if property == "tech" => update_project_tech(project),
        property if property == "location" => update_project_location(project),
        property if property == "repository" => update_project_repository(project),
        property if property == "github_project" => update_project_github_project(project),
        property if property == "version" => update_project_version(project),
        _ => {
            println!("no property matching detected.");
            project
        }
    }
}

fn update_project_id(project: projects::Project) -> projects::Project {
    let new_id = utils::prompt_message(
        "Enter the new project's id: ".to_string(),
        "Error getting the new project's id".to_string(),
    );
    let update_project: projects::Project = projects::Project {
        id: new_id.to_lowercase(),
        name: project.name,
        tech: project.tech,
        location: project.location,
        repository: project.repository,
        github_project: project.github_project,
        version: project.version,
    };
    return update_project;
}

fn update_project_name(project: projects::Project) -> projects::Project {
    let new_name = utils::prompt_message(
        "Enter the new project's name: ".to_string(),
        "Error getting the new project's name".to_string(),
    );
    let update_project: projects::Project = projects::Project {
        id: project.id,
        name: new_name,
        tech: project.tech,
        location: project.location,
        repository: project.repository,
        github_project: project.github_project,
        version: project.version,
    };
    return update_project;
}

fn update_project_location(project: projects::Project) -> projects::Project {
    let new_location = utils::prompt_message(
        "Enter the new project's location: ".to_string(),
        "Error getting the new project's location".to_string(),
    );
    let re = Regex::new(r"^(/[^/ ]*)+/?$").unwrap();
    if !re.is_match(&new_location) {
        println!(
            "{}",
            "The location path is not correct. Please enter a correct path."
        )
    }
    let update_project: projects::Project = projects::Project {
        id: project.id,
        name: project.name,
        tech: project.tech,
        location: new_location,
        repository: project.repository,
        github_project: project.github_project,
        version: project.version,
    };
    return update_project;
}

fn update_project_tech(project: projects::Project) -> projects::Project {
    let new_tech = utils::get_select_option(
        "Select the new project's tech".to_string(),
        utils::get_tech_option(),
    )
    .expect("Failed to select the new project's tech");
    let update_project: projects::Project = projects::Project {
        id: project.id,
        name: project.name,
        tech: new_tech,
        location: project.location,
        repository: project.repository,
        github_project: project.github_project,
        version: project.version,
    };
    return update_project;
}

fn update_project_repository(project: projects::Project) -> projects::Project {
    let new_url = utils::prompt_message(
        "Enter the new project's repository: ".to_string(),
        "Error getting the new project's repository".to_string(),
    );
    let re = Regex::new(r"https?:\/\/(www\.)?[-a-zA-Z0-9@:%._\+~#=]{1,256}\.[a-zA-Z0-9()]{1,6}\b([-a-zA-Z0-9()@:%_\+.~#?&//=]*)").unwrap();
    if !re.is_match(&new_url) {
        println!("{}", "The url is not correct. Please enter a correct url.")
    }
    let update_project: projects::Project = projects::Project {
        id: project.id,
        name: project.name,
        tech: project.tech,
        location: project.location,
        repository: new_url,
        github_project: project.github_project,
        version: project.version,
    };
    return update_project;
}

fn update_project_github_project(project: projects::Project) -> projects::Project {
    let new_url = utils::prompt_message(
        "Enter the new project's github project: ".to_string(),
        "Error getting the new project's github project".to_string(),
    );
    let re = Regex::new(r"https?:\/\/(www\.)?[-a-zA-Z0-9@:%._\+~#=]{1,256}\.[a-zA-Z0-9()]{1,6}\b([-a-zA-Z0-9()@:%_\+.~#?&//=]*)").unwrap();
    if !re.is_match(&new_url) {
        println!("{}", "The url is not correct. Please enter a correct url.")
    }
    let update_project: projects::Project = projects::Project {
        id: project.id,
        name: project.name,
        tech: project.tech,
        location: project.location,
        repository: project.repository,
        github_project: new_url,
        version: project.version,
    };
    return update_project;
}

fn update_project_version(project: projects::Project) -> projects::Project {
    let new_version = utils::prompt_message(
        "Enter the new project's version: ".to_string(),
        "Error getting the new project's version".to_string(),
    );
    let update_project: projects::Project = projects::Project {
        id: project.id,
        name: project.name,
        tech: project.tech,
        location: project.location,
        repository: project.repository,
        github_project: project.github_project,
        version: new_version,
    };
    return update_project;
}
