/*!
Module to handle the NXS (nyx storage) binary format to store projects data.

This module provides the definition of data structures and methods to create and handle the binary format used for storing project data in the NXS format.
It includes functionalities for reading, writing, and manipulating the binary data to ensure efficient storage and retrieval.
*/

use crate::utils;

use bincode;
use serde::Serialize;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::{fs, vec};

pub struct NXSFile {
    pub header: Header,
    pub project_list: ProjectList,
}

pub struct Header {
    magic_number: &'static [u8],
    format_version: &'static [u8],
    project_count: u8,
}

#[derive(Debug, Clone, Serialize)]
pub struct ProjectList {
    pub entries: Vec<ProjectEntry>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ProjectEntry {
    pub project_hash: String,
    pub project_id: String,
    pub project_size: u32,
}

pub fn create_data() {
    utils::change_work_dir(&utils::get_nyx_env_var());
    if Path::new(".data").exists() {
        lrncore::logs::info_log("Reinitialized data folder");
        let remove_dir = fs::remove_dir_all(".data");
        if let Err(e) = remove_dir {
            lrncore::logs::error_log(&format!("Failed to remove existing .data directory: {}", e));
        }
    }
    let data_folder = match std::fs::create_dir(".data") {
        Ok(_) => (),
        Err(e) => {
            lrncore::logs::error_log(&format!("Failed to remove existing .data directory: {}", e));
        }
    };
    create_nxs_file();
}

fn create_nxs_file() {
    // header
    let header: Header = Header {
        magic_number: b"NXS\0",
        format_version: b"0.1.0\0",
        project_count: 0,
    };
    let mut header_buff: Vec<u8> = Vec::new();
    header_buff.extend_from_slice(header.magic_number);
    header_buff.extend_from_slice(header.format_version);
    header_buff.push(header.project_count);
    header_buff.extend_from_slice(b" ");
    // project list
    let project_list: ProjectList = ProjectList {
        entries: Vec::new(),
    };
    let mut project_list_buff: Vec<u8> = Vec::new();
    //TODO
    // update bincode version and refactor binary module to use new version serialize
    let project_list_bytes =
        bincode::serialize(&project_list).expect("Failed to serialize project list");
    project_list_buff.extend_from_slice(&project_list_bytes);
    // complete file
    let file: NXSFile = NXSFile {
        header: header,
        project_list: project_list,
    };
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
    let mut nxs_file_write = match nxs_file.write_all(&file_buff) {
        Ok(_) => (),
        Err(e) => {
            lrncore::logs::error_log(&format!("Failed to write buffer in nxs file: {}", e));
        }
    };
    lrncore::logs::info_log("Initialized NXS file");
}
