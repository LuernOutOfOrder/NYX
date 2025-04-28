use std::{env, process::exit};
mod helper;

use chrono::{Datelike, Month};
use lrncore::usage_exit::command_usage;
use systemstat::{saturating_sub_bytes, Platform, System};

use crate::{
    nxfs::{self, config::LogLevel},
    utils,
};
use utils::log;
pub fn hello_help() -> String {
    let usage = r"
Usage: nyx hello [options]

Options:

    -h, --help      Show this help message
";

    usage.to_string()
}

pub fn hello_command() {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 2 {
        hello();
        exit(0);
    }
    match args[2].as_str() {
        "-h" => {
            command_usage(&hello_help());
        }
        "--help" => {
            command_usage(&hello_help());
        }
        _ => {
            log::log_from_log_level(LogLevel::Warn, "Unknown command");
            command_usage(&hello_help());
        }
    }
}

fn hello() {
    let user_name: String = nxfs::config::parse_config_file()
        .expect("Failed to get config file")
        .user
        .name;
    // welcome
    println!(
        "Welcome {}\t({}/{})",
        user_name,
        std::env::consts::OS,
        std::env::consts::ARCH
    );
    // date
    let date = chrono::Local::now();
    let month = Month::try_from(u8::try_from(date.month()).unwrap()).ok();
    println!(
        "\tSystem information as of {} {:?} {} {}\n",
        date.weekday(),
        month.unwrap(),
        date.day(),
        date.time()
    );

    // Display system information:
    let sys = System::new();
    match sys.load_average() {
        Ok(loadavg) => println!("\tSystem load average: {}", loadavg.one),
        Err(x) => println!("\tSystem load average: error: {}", x),
    }
    match sys.mount_at("/") {
        Ok(mount) => {
            print!("\tUsage of /:\t");
            println!("{} of {}", mount.avail, mount.total);
        }
        Err(x) => println!("\tMount at /: error: {}", x),
    }
    match sys.memory() {
        Ok(mem) => println!(
            "\tMemory: {} used / {} ",
            saturating_sub_bytes(mem.total, mem.free),
            mem.total
        ),
        Err(x) => println!("\tMemory: error: {}", x),
    }
    match sys.swap() {
        Ok(swap) => println!(
            "\tSwap: {} used / {}",
            saturating_sub_bytes(swap.total, swap.free),
            swap.total
        ),
        Err(x) => println!("\tSwap: error: {}", x),
    }
    match sys.cpu_temp() {
        Ok(cpu_temp) => println!("\tCPU temp: {}", cpu_temp),
        Err(x) => println!("\tCPU temp: {}", x),
    }

    println!("\nNXFS information: ");
    let nyx_path = utils::env::get_nyx_env_var();
    let nxfs_path = nyx_path.clone() + "/.nxfs/";
    print!("\tNxfs directory size: {}", helper::folder_size(&nxfs_path));
    let nxfs_projects_path = nyx_path.clone() + "/.nxfs/projects/";
    print!(
        "\tProjects directory size: {}",
        helper::folder_size(&nxfs_projects_path)
    );
    let nxfs_temp_path = nyx_path + "/.nxfs/tmp/";
    print!(
        "\tTemp directory size: {}",
        helper::folder_size(&nxfs_temp_path)
    );
    println!(
        "\tNumber of projects: {}",
        nxfs::nxs::get_all_project().len()
    );
}
