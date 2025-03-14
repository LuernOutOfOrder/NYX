/*!
Module to handle the NXS (nyx storage) and NXP (nyx project) binary format to store projects data.

This module provides the definition of data structures and methods to create and handle the binary format used for storing project data in the NXS format and the NXP format.
It includes functionalities for reading, writing, and manipulating the binary data to ensure efficient storage and retrieval.
*/

use crate::utils;

use bincode;
use serde::{Deserialize, Serialize};
use std::fs;
use std::fs::File;
use std::io::{BufReader, Read, Write};
use std::path::Path;

/*
NXS file structure
*/

#[derive(Debug, Deserialize)]
struct Header {
    magic_number: [u8; 4],
    format_version: [u8; 6],
    project_size: u32,
    project_count: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ProjectList {
    pub entries: Vec<ProjectEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ProjectEntry {
    pub project_hash: [u8; 20],
    pub project_id: Vec<u8>,
    pub project_size: u32,
}

/*
NXP file structure
*/
struct Project {}

pub fn create_data() {
    utils::change_work_dir(&utils::get_nyx_env_var());
    if Path::new(".data").exists() {
        lrncore::logs::info_log("Reinitialized data folder");
        let remove_dir = fs::remove_dir_all(".data");
        if let Err(e) = remove_dir {
            lrncore::logs::error_log(&format!("Failed to remove existing .data directory: {}", e));
        }
    }
    match std::fs::create_dir(".data") {
        Ok(_) => create_nxs_file(),
        Err(e) => {
            lrncore::logs::error_log(&format!("Failed to remove existing .data directory: {}", e));
        }
    };
    parse_nxs_file();
}

fn create_nxs_file() {
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
    // header
    let header: Header = Header {
        magic_number: *b"NXS\0",
        format_version: *b"0.1.0\0",
        project_size: bincode::serialized_size(&project_list)
            .expect("Failed to calculate serialized size") as u32,
        project_count: 0,
    };
    let mut header_buff: Vec<u8> = Vec::new();
    header_buff.extend_from_slice(&header.magic_number);
    header_buff.extend_from_slice(&header.format_version);
    header_buff.push(header.project_count);
    header_buff.extend_from_slice(b" ");
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

fn parse_nxs_file() {
    utils::change_work_dir(&utils::get_nyx_env_var());
    // open NXS file and match result
    let mut file = match File::open(".data/nxs") {
        Ok(f) => f,
        Err(e) => {
            lrncore::logs::error_log(&format!("Failed to open nxs file: {}", e));
            return;
        }
    };
    // initialize header size from structure and buffer
    let header_size = std::mem::size_of::<Header>();
    let buffer = BufReader::new(file);
    // vector containing the whole NXS file
    let mut bytes_vec: Vec<u8> = Vec::new();
    for byte_or_error in buffer.bytes() {
        let byte = byte_or_error.unwrap();
        bytes_vec.push(byte);
    }

    // extract a slice of bytes from the `bytes_vec` vector to represent the header section of the NXS file.
    // &bytes_vec[0 to header_size]
    let header_bytes = &bytes_vec[..header_size];

    // convert into the Header struct
    let header: Header = bincode::deserialize(header_bytes).expect("Failed to deserialize header");
    println!("{:?}", String::from_utf8_lossy(&bytes_vec));
    // project list
    let project_list_size = header.project_size;
    let project_list_byte = &bytes_vec[header_size..];
    let project_list: ProjectList =
        bincode::deserialize(project_list_byte).expect("Failed to deserialize project list");
    println!("{:?}", project_list.entries);
}
