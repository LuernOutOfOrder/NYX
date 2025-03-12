/*!
Module to handle the NXS (nyx storage) binary format to store projects data.

This module provides the definition of data structures and methods to create and handle the binary format used for storing project data in the NXS format.
It includes functionalities for reading, writing, and manipulating the binary data to ensure efficient storage and retrieval.
*/

use crate::utils;

use std::fs;
use std::path::Path;

pub struct NXSFile {
    pub header: Header,
    pub project_list: ProjectList,
}

pub struct Header {
    magic_number: u32,
    project_count: u32,
    project_list: u32,
    format_version: u8,
}

#[derive(Debug, Clone)]
pub struct ProjectList {
    pub entries: Vec<ProjectEntry>,
}

#[derive(Debug, Clone)]
pub struct ProjectEntry {
    pub project_hash: String,
    pub project_id: String,
    pub project_size: u32,
}

pub fn create_nxs() {
    utils::change_work_dir(&utils::get_nyx_env_var());
    if Path::new(".data").exists() {
        lrncore::logs::info_log("Reinitialized data folder");
        let remove_dir = fs::remove_dir_all(".data");
        if let Err(e) = remove_dir {
            lrncore::logs::error_log(&format!("Failed to remove existing .data directory: {}", e));
        }
    }
    let data_folder = std::fs::create_dir(".data");
    match data_folder {
        Ok(_) => (),
        Err(e) => {
            lrncore::logs::error_log(&format!("Failed to remove existing .data directory: {}", e));
        }
    }
}
