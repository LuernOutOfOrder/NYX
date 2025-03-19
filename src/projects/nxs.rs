/*!
Module to handle the NXS (nyx storage) binary format to store projects data.

This module provides the definition of data structures and methods to create and handle the binary format used for storing project data in the NXS format and the NXP format.
It includes functionalities for reading, writing, and manipulating the binary data to ensure efficient storage and retrieval.
*/

use crate::projects::nxp;
use crate::utils;

use bincode;
use serde::{Deserialize, Serialize};
use std::fs;
use std::fs::File;
use std::io::{BufReader, Read, Write};
use std::path::Path;

use super::nxp::{NXPContent, NXP};

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
#[derive(Debug, Deserialize, Clone)]
struct NXS {
    header: NXSHeader,
    projects: ProjectList,
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
#[derive(Debug, Deserialize, Clone)]
struct NXSHeader {
    magic_number: [u8; 4],
    format_version: [u8; 6],
    project_count: u8,
    #[allow(dead_code)]
    reserved: u32,
}

/// The `ProjectList` struct contains a vector of `ProjectEntry` instances.
///
/// Properties:
///
/// * `entries`: The `entries` property in the `ProjectList` struct is a vector of `ProjectEntry`
/// instances. It represents a list of project entries within the project list.
#[derive(Debug, Clone, Serialize, Deserialize)]
struct ProjectList {
    pub entries: Vec<ProjectEntry>,
}

/// The `ProjectEntry` struct in Rust contains fields for project hash, project ID, and project size.
///
/// Properties:
///
/// * `project_hash`: The `project_hash` property in the `ProjectEntry` struct is defined as an array of
/// 20 unsigned 8-bit integers (bytes). This array represents a hash value typically used to uniquely
/// identify a project.
/// * `project_id`: The `project_id` property in the `ProjectEntry` struct is a vector of unsigned 8-bit
/// integers (`Vec<u8>`). It is used to store the unique identifier for a project.
/// * `project_size`: The `project_size` property in the `ProjectEntry` struct represents the size of
/// the project in bytes. It is of type `u32`, which means it can hold unsigned integer values ranging
/// from 0 to 2^32 - 1. This property indicates the amount of storage space the
#[derive(Debug, Clone, Serialize, Deserialize)]
struct ProjectEntry {
    pub project_hash: [u8; 20],
    pub project_id: Vec<u8>,
    pub project_size: u32,
}

/// The `create_data` function initializes a data folder, removes any existing data directory,
/// creates a new data directory, and parses a NXS file.
pub fn create_data() {
    utils::change_work_dir(&utils::get_nyx_env_var());
    if Path::new(".data").exists() {
        lrncore::logs::info_log("Reinitialized data folder");
        let remove_dir = fs::remove_dir_all(".data");
        if let Err(e) = remove_dir {
            lrncore::logs::error_log(&format!("Failed to remove existing .data directory: {}", e));
        }
    }
    match std::fs::create_dir_all(".data/projects") {
        Ok(_) => create_nxs_file(),
        Err(e) => {
            lrncore::logs::error_log(&format!("Failed to remove existing .data directory: {}", e));
        }
    };

    let mut nxs = NXS {
        header: NXSHeader {
            magic_number: [0; 4],
            format_version: [0; 6],
            project_count: 0,
            reserved: 0,
        },
        projects: ProjectList { entries: vec![] },
    };
    parse_nxs_file(&mut nxs);
    let content: NXPContent = NXPContent {
        name: "Testing project test".to_string(),
        tech: String::new(),
        location: String::new(),
        repository: String::new(),
        github_project: String::new(),
        version: String::new(),
        todo: String::new(),
    };
    nxp::create_new_nxp(content);
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
        project_hash: [0; 20],
        project_id: vec![],
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
    let mut nxs_file: File = match File::create(".data/nxs") {
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

fn parse_nxs_file(nxs_ref: &mut NXS) {
    utils::change_work_dir(&utils::get_nyx_env_var());
    // open NXS file and match result
    let file = match File::open(".data/nxs") {
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

pub fn update_nxs_file(nxp: &mut NXP) {
    let mut empty_nxs: NXS = NXS {
        header: NXSHeader {
            magic_number: [0; 4],
            format_version: [0; 6],
            project_count: 0,
            reserved: 0,
        },
        projects: ProjectList { entries: vec![] },
    };
    parse_nxs_file(&mut empty_nxs);
    println!("nxs {:?}", empty_nxs);
}
