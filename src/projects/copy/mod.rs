use std::{
    env,
    process::{exit, Command, Stdio},
};

use lrncore::{path::change_work_dir, usage_exit::command_usage};

use crate::{
    nxfs::{
        config::LogLevel,
        nxp::{self, NXPContent, NXPHeader, NXP},
        nxs::{self},
    },
    utils::{self, env::get_nyx_env_var, log::log_from_log_level},
};

pub fn copy_help() -> String {
    let usage = r"
Usage: copy [subcommand] [arguments] [options]

Subcommands:
    path        Copy the path of specified project
    repo        Copy the repository url of specified project

Options:
    -h, --help      Show this help message
    ";

    usage.to_string()
}

pub fn copy_command() {
    change_work_dir(&get_nyx_env_var());
    let args: Vec<String> = env::args().collect();
    if args.len() <= 3 {
        command_usage(&copy_help());
        return;
    }
    if args.len() > 1 && (args[3] == "-h" || args[3] == "--help") {
        command_usage(&copy_help());
        return;
    }

    if let Some(arg) = args.get(3) {
        let get_project = get_project_content();
        match arg.as_str().trim() {
            "path" => copy_field(get_project.location),
            "repo" => copy_field(get_project.repository),
            _ => {
                log_from_log_level(LogLevel::Error, "Unknown copy command");
                command_usage(&copy_help());
            }
        }
    }
}

fn copy_field(param: String) {
    let mut pbcopy = Command::new("pbcopy")
        .stdin(Stdio::piped())
        .spawn()
        .expect("Failed to spawn pbcopy command");
    let wait_pbcopy = pbcopy.wait().expect("Failed to wait pbcopy command");
    if !wait_pbcopy.success() {
        log_from_log_level(LogLevel::Error, "Failed to execute pbcopy command");
    }
    Command::new("echo")
        .arg(param)
        .stdout(pbcopy.stdin.unwrap())
        .output()
        .expect("Failed to execute echo command");
    log_from_log_level(LogLevel::Info, "Project field copied in clipboard");
}

fn get_project_content() -> NXPContent {
    let mut projects = nxs::get_all_project();
    inquire::set_global_render_config(utils::get_render_config());
    let app_name = utils::prompt_message(
        "Enter project name:".to_string(),
        "Error with the project name referred".to_string(),
    );
    let project_hash: [u8; 11];
    if let Some(pos) = projects.iter().position(|app| app.project_name == app_name) {
        log_from_log_level(LogLevel::Info, "Project found");
        let app = projects.remove(pos);
        project_hash = app.project_hash;
    } else {
        log_from_log_level(LogLevel::Error, "Project not found");
        exit(10);
    }
    let mut nxp: NXP = NXP {
        header: NXPHeader {
            magic_number: [0; 4],
            format_version: [0; 6],
            project_id: [0; 11],
            project_size: 0,
            reserved: 0,
        },
        content: NXPContent {
            name: String::new(),
            tech: String::new(),
            location: String::new(),
            repository: String::new(),
            github_project: String::new(),
            version: String::new(),
        },
    };
    let hash = String::from_utf8_lossy(&project_hash);
    nxp::parse_nxp_file(&format!(".nxfs/projects/{}/content", &hash), &mut nxp);
    let project_content: NXPContent = nxp.content;
    project_content
}
