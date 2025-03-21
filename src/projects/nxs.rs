/*!
Module to handle the NXS (nyx storage) binary format to store projects data.

This module provides the definition of data structures and methods to create and handle the binary format used for storing project data in the NXS format and the NXP format.
It includes functionalities for reading, writing, and manipulating the binary data to ensure efficient storage and retrieval.
*/

use crate::utils;

use bincode;
use lrncore::logs::time_info_log;
use serde::{Deserialize, Serialize};
use std::fs;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::{BufReader, Read, Write};
use std::path::Path;
use std::process::exit;
use std::process::Command;

use super::nxp::parse_nxp_file;
use super::nxp::NXPContentShort;
use super::nxp::NXP;
use crate::projects::nxp::{NXPContent, NXPHeader};

/*
NXS file structure
*/

/// The code defines a struct `NXS` containing a `NXSHeader` and a `ProjectList`.
///
/// Properties:
///
/// * `NXSHeader`: The `NXSHeader` property in the `NXS` struct is of type `NXSHeader`. It likely contains
/// information about the overall structure or metadata of the NXS data.
/// * `projects`: The `projects` property in the `NXS` struct is of type `ProjectList`. It likely
/// represents a list of projects within the NXS structure.
#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct NXS {
    pub header: NXSHeader,
    pub projects: ProjectList,
}

/// The `NXSHeader` struct in Rust represents a data structure with fields for magic number, format
/// version, project count, and a reserved value.
///
/// Properties:
///
/// * `magic_number`: The `magic_number` field in the `NXSHeader` struct is an array of 4 unsigned 8-bit
/// integers (`u8`). It is typically used to identify the file format or type by storing a specific
/// sequence of bytes that can be checked when reading the file to ensure it is of the expected
/// * `format_version`: The `format_version` property in the `NXSHeader` struct is an array of 6 unsigned
/// 8-bit integers (`[u8; 6]`). This array is used to store the version information of the format being
/// used. Each element in the array represents a part of the version number.
/// * `project_count`: The `project_count` property in the `NXSHeader` struct represents the number of
/// projects stored in the data structure. It is of type `u8`, which means it can hold values from 0 to
/// 255.
/// * `reserved`: The `reserved` field in the `NXSHeader` struct is a 32-bit unsigned integer that is
/// currently marked with `#[allow(dead_code)]`. This attribute is used to suppress the compiler warning
/// about unused code, indicating that the field is intentionally left unused or reserved for future
/// use.
#[derive(Debug, Deserialize, Clone, Serialize)]
#[repr(C, packed)]
pub struct NXSHeader {
    pub magic_number: [u8; 4],
    pub format_version: [u8; 6],
    pub project_count: u8,
    #[allow(dead_code)]
    pub reserved: u32,
}

/// The `ProjectList` struct contains a vector of `ProjectEntry` instances.
///
/// Properties:
///
/// * `entries`: The `entries` property in the `ProjectList` struct is a vector of `ProjectEntry`
/// instances. It represents a list of project entries within the project list.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectList {
    pub entries: Vec<ProjectEntry>,
}

/// The `ProjectEntry` struct in Rust contains fields for project hash, project ID, and project size.
///
/// Properties:
///
/// * `project_hash`: The `project_hash` property in the `ProjectEntry` struct is defined as an array of
/// 20 unsigned 8-bit integers (bytes). This array represents a hash value typically used to uniquely
/// identify a project.
/// * `project_name`: The `project_name` property in the `ProjectEntry` struct is a vector of unsigned 8-bit
/// integers (`Vec<u8>`). It is used to store the unique identifier for a project.
/// * `project_size`: The `project_size` property in the `ProjectEntry` struct represents the size of
/// the project in bytes. It is of type `u32`, which means it can hold unsigned integer values ranging
/// from 0 to 2^32 - 1. This property indicates the amount of storage space the
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectEntry {
    pub project_name: String,
    pub project_hash: [u8; 11],
    pub project_size: u32,
}

/// The `create_data` function initializes a data folder, removes any existing data directory,
/// creates a new data directory, and parses a NXS file.
pub fn create_data() {
    utils::change_work_dir(&utils::get_nyx_env_var());
    if Path::new(".nxfs").exists() {
        lrncore::logs::info_log("Reinitialized data folder");
        let remove_dir = fs::remove_dir_all(".nxfs");
        if let Err(e) = remove_dir {
            lrncore::logs::error_log(&format!("Failed to remove existing .nxfs directory: {}", e));
        }
    }
    let mut mkdir = Command::new("mkdir")
        .arg(".nxfs")
        .arg(".nxfs/projects")
        .arg(".nxfs/tmp")
        .spawn()
        .expect("Failed to create all directories");
    let wait_mkdir = mkdir.wait().expect("Failed to wait mkdir command");
    if !wait_mkdir.success() {
        lrncore::logs::error_log("Failed to execute mkdir command");
        exit(1)
    }
    create_nxs_file()
}

/// The function `create_nxs_file` creates a NXS file with a NXSHeader and project list.
fn create_nxs_file() {
    // NXSHeader
    let header: NXSHeader = NXSHeader {
        reserved: 0,
        magic_number: *b"NXS\0",
        format_version: *b"0.1.0\0",
        project_count: 0,
    };

    let mut header_buff: Vec<u8> = Vec::new();
    header_buff.extend_from_slice(&header.magic_number);
    header_buff.extend_from_slice(&header.format_version);
    header_buff.push(header.project_count);
    header_buff.extend_from_slice(b" ");
    // project list
    let test: ProjectEntry = ProjectEntry {
        project_name: String::new(),
        project_hash: [0; 11],
        project_size: 0,
    };
    let empty_vec: Vec<ProjectEntry> = vec![test];
    let project_list: ProjectList = ProjectList { entries: empty_vec };
    let mut project_list_buff: Vec<u8> = Vec::new();
    //TODO
    // update bincode version and refactor binary module to use new version serialize
    let project_list_bytes =
        bincode::serialize(&project_list).expect("Failed to serialize project list");
    project_list_buff.extend_from_slice(&project_list_bytes);
    // complete file
    let mut file_buff: Vec<u8> = Vec::new();
    file_buff.extend_from_slice(&header_buff);
    file_buff.extend_from_slice(&project_list_buff);
    let mut nxs_file: File = match File::create(".nxfs/nxs") {
        Ok(f) => f,
        Err(e) => {
            lrncore::logs::error_log(&format!("Failed to create nxs file: {}", e));
            return;
        }
    };
    match nxs_file.write_all(&file_buff) {
        Ok(_) => (),
        Err(e) => {
            lrncore::logs::error_log(&format!("Failed to write buffer in nxs file: {}", e));
        }
    };
    lrncore::logs::info_log("Initialized NXS file");
}

//TODO
// refactor to return a structure instead of a ref
fn parse_nxs_file(nxs_ref: &mut NXS) {
    utils::change_work_dir(&utils::get_nyx_env_var());
    // open NXS file and match result
    let file = match File::open(".nxfs/nxs") {
        Ok(f) => f,
        Err(e) => {
            lrncore::logs::error_log(&format!("Failed to open nxs file: {}", e));
            return;
        }
    };
    // initialize NXSHeader size from structure and buffer
    let header_size = std::mem::size_of::<NXSHeader>();
    let buffer = BufReader::new(file);
    // vector containing the whole NXS file
    let mut bytes_vec: Vec<u8> = Vec::new();
    for byte_or_error in buffer.bytes() {
        let byte = byte_or_error.unwrap();
        bytes_vec.push(byte);
    }

    // extract a slice of bytes from the `bytes_vec` vector to represent the NXSHeader section of the NXS file.
    // &bytes_vec[0 to NXSHeader_size]
    let header_bytes = &bytes_vec[..header_size];

    // convert into the NXSHeader struct
    let header: NXSHeader =
        bincode::deserialize(header_bytes).expect("Failed to deserialize NXSHeader");
    // project list
    let project_list_byte = &bytes_vec[header_size..];
    let project_list: ProjectList =
        bincode::deserialize(project_list_byte).expect("Failed to deserialize project list");
    let nxs: NXS = NXS {
        header: header,
        projects: project_list,
    };
    nxs_ref.header = nxs.header;
    nxs_ref.projects = nxs.projects;
}

pub fn update_nxs_file(nxp_ref: &mut NXP) {
    utils::change_work_dir(&utils::get_nyx_env_var());
    let mut nxs: NXS = NXS {
        header: NXSHeader {
            magic_number: [0; 4],
            format_version: [0; 6],
            project_count: 0,
            reserved: 0,
        },
        projects: ProjectList { entries: vec![] },
    };
    parse_nxs_file(&mut nxs);
    let new_entry: ProjectEntry = new_project_entry(
        &nxp_ref.header.project_id,
        &nxp_ref.content.name,
        nxp_ref.header.project_size,
    );
    let mut nxs_entries: ProjectList = nxs.projects;
    nxs_entries.entries.push(new_entry);
    nxs.projects = nxs_entries;
    let file_buff = bincode::serialize(&nxs).expect("Failed to serialize updated NXS file");
    let mut nxs_file: File = match OpenOptions::new()
        .write(true)
        .create(false)
        .truncate(true)
        .open(".nxfs/nxs")
    {
        Ok(f) => f,
        Err(e) => {
            lrncore::logs::error_log(&format!("Failed to open nxs file: {}", e));
            return;
        }
    };
    match nxs_file.write_all(&file_buff) {
        Ok(_) => (),
        Err(e) => {
            lrncore::logs::error_log(&format!("Failed to write buffer in nxs file: {}", e));
        }
    };
    time_info_log("NXS file updated");
}

pub fn update_project_entries(nxs_ref: &mut NXS, vec: Vec<ProjectEntry>) {
    utils::change_work_dir(&utils::get_nyx_env_var());
    parse_nxs_file(nxs_ref);
    nxs_ref.projects.entries = vec;
    let file_buff = bincode::serialize(&nxs_ref).expect("Failed to serialize updated NXS file");
    let mut nxs_file: File = match OpenOptions::new()
        .write(true)
        .create(false)
        .truncate(true)
        .open(".nxfs/nxs")
    {
        Ok(f) => f,
        Err(e) => {
            lrncore::logs::error_log(&format!("Failed to open nxs file: {}", e));
            return;
        }
    };
    match nxs_file.write_all(&file_buff) {
        Ok(_) => (),
        Err(e) => {
            lrncore::logs::error_log(&format!("Failed to write buffer in nxs file: {}", e));
        }
    };
    time_info_log("NXS file updated");
}

fn new_project_entry(hash: &[u8; 11], id: &String, size: u32) -> ProjectEntry {
    let new: ProjectEntry = ProjectEntry {
        project_name: id.clone(),
        project_hash: *hash,
        project_size: size,
    };
    new
}

pub fn cat_nxs() {
    utils::change_work_dir(&utils::get_nyx_env_var());
    let mut nxs: NXS = NXS {
        header: NXSHeader {
            magic_number: [0; 4],
            format_version: [0; 6],
            project_count: 0,
            reserved: 0,
        },
        projects: ProjectList { entries: vec![] },
    };
    parse_nxs_file(&mut nxs);
    lrncore::logs::info_log("Listing all projects entries");
    println!(
        "
NXS:
    projects: 
",
    );
    for each in nxs.projects.entries {
        println!(
            "name: {:?}\n hash: {:?}\n size: {:?}\n",
            each.project_name,
            String::from_utf8_lossy(&each.project_hash),
            each.project_size
        );
    }
}

pub fn get_all_project_entries() -> Vec<NXPContent> {
    utils::change_work_dir(&utils::get_nyx_env_var());
    let mut nxs: NXS = NXS {
        header: NXSHeader {
            magic_number: [0; 4],
            format_version: [0; 6],
            project_count: 0,
            reserved: 0,
        },
        projects: ProjectList { entries: vec![] },
    };
    parse_nxs_file(&mut nxs);
    let mut project_vec: Vec<NXPContent> = Vec::new();
    for each in nxs.projects.entries {
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
                todo: String::new(),
            },
        };
        parse_nxp_file(
            &format!(
                ".nxfs/projects/{}",
                String::from_utf8_lossy(&each.project_hash)
            ),
            &mut nxp,
        );
        project_vec.push(nxp.content);
    }
    project_vec
}

pub fn get_all_short_project() -> Vec<NXPContentShort> {
    utils::change_work_dir(&utils::get_nyx_env_var());
    let mut nxs: NXS = NXS {
        header: NXSHeader {
            magic_number: [0; 4],
            format_version: [0; 6],
            project_count: 0,
            reserved: 0,
        },
        projects: ProjectList { entries: vec![] },
    };
    parse_nxs_file(&mut nxs);
    let mut project_vec: Vec<NXPContentShort> = Vec::new();
    for each in nxs.projects.entries {
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
                todo: String::new(),
            },
        };
        parse_nxp_file(
            &format!(
                ".nxfs/projects/{}",
                String::from_utf8_lossy(&each.project_hash)
            ),
            &mut nxp,
        );
        let short_content: NXPContentShort = NXPContentShort {
            name: nxp.content.name,
            tech: nxp.content.tech,
            location: nxp.content.location,
            version: nxp.content.version,
        };
        project_vec.push(short_content);
    }
    project_vec
}

pub fn get_all_project() -> Vec<ProjectEntry> {
    utils::change_work_dir(&utils::get_nyx_env_var());
    let mut nxs: NXS = NXS {
        header: NXSHeader {
            magic_number: [0; 4],
            format_version: [0; 6],
            project_count: 0,
            reserved: 0,
        },
        projects: ProjectList { entries: vec![] },
    };
    parse_nxs_file(&mut nxs);
    nxs.projects.entries
}
