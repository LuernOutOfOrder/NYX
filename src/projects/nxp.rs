/*!
Module to handle the NXP (nyx project) binary format to store projects data.

This module provides the definition of data structures and methods to create and handle the binary format used for storing project data in the NXS format and the NXP format.
It includes functionalities for reading, writing, and manipulating the binary data to ensure efficient storage and retrieval.
*/

use crate::utils;
use serde::Deserialize;
use serde::Serialize;
use sha1::{Digest, Sha1};
use std::fs::File;
use std::io::BufReader;
use std::io::Read;
use std::io::Write;

/*
NXP file structure
*/
#[derive(Debug, Deserialize, Serialize)]
struct NXP {
    header: NXPHeader,
    content: NXPContent,
}

#[derive(Debug, Deserialize, Serialize)]
#[repr(C, packed)]
struct NXPHeader {
    magic_number: [u8; 4],
    format_version: [u8; 6],
    project_id: [u8; 11],
    project_size: u32,
    reserved: u32,
}

#[derive(Serialize, Deserialize, Debug)]
struct NXPContent {
    name: String,
    tech: String,
    location: String,
    repository: String,
    github_project: String,
    version: String,
    todo: String,
}

pub fn create_new_nxp() {
    utils::change_work_dir(&utils::get_nyx_env_var());
    // hash
    let mut new_hash = Sha1::new();
    new_hash.update("prout".to_owned());
    let hash_result = new_hash.finalize();
    let folder_hash = format!("{:#x}", hash_result);
    let (file_hash, _) = folder_hash.split_at(11);
    // content
    let content: NXPContent = NXPContent {
        name: format!("{}", "testing project"),
        tech: format!("{}", "Rust"),
        location: format!("{}", "empty"),
        repository: format!("{}", "empty"),
        github_project: format!("{}", "empty"),
        version: format!("{}", "0.1.0"),
        todo: format!("{}", "empty"),
    };
    let content_buff = bincode::serialize(&content).expect("Failed to serialize content buffer");
    // header
    let header: NXPHeader = NXPHeader {
        magic_number: *b"NXP\0",
        format_version: *b"0.1.0\0",
        project_id: {
            let mut array = [0u8; 11];
            let bytes = file_hash.as_bytes();
            array.copy_from_slice(&bytes[..11]);
            array
        },
        project_size: content_buff.len() as u32,
        reserved: 0,
    };
    let header_buff: Vec<u8> = bincode::serialize(&header).expect("Failed to serialize NXP header");
    // complete file
    let mut file_buff: Vec<u8> = Vec::new();
    file_buff.extend_from_slice(&header_buff);
    file_buff.extend_from_slice(&content_buff);
    let file_path = format!(".data/projects/{}", file_hash);
    let mut nxs_file: File = match File::create(file_path) {
        Ok(f) => f,
        Err(e) => {
            lrncore::logs::error_log(&format!("Failed to create nxp file: {}", e));
            return;
        }
    };
    match nxs_file.write_all(&file_buff) {
        Ok(_) => (),
        Err(e) => {
            lrncore::logs::error_log(&format!("Failed to write buffer in nxs file: {}", e));
        }
    };
    lrncore::logs::info_log("Initialized NXP file");
}

pub fn parse_nxp_file(path: &str) {
    utils::change_work_dir(&utils::get_nyx_env_var());
    let file = match File::open(path) {
        Ok(f) => f,
        Err(e) => {
            lrncore::logs::error_log(&format!("Failed to open nxp file: {}", e));
            return;
        }
    };
    // initialize NXPHeader size from structure and buffer
    let header_size = std::mem::size_of::<NXPHeader>();
    let buffer = BufReader::new(file);
    // vector containing the whole NXP file
    let mut bytes_vec: Vec<u8> = Vec::new();
    for byte_or_error in buffer.bytes() {
        match byte_or_error {
            Ok(byte) => bytes_vec.push(byte),
            Err(e) => {
                lrncore::logs::error_log(&format!("Failed to read byte: {}", e));
                return;
            }
        }
    }

    // extract a slice of bytes from the `bytes_vec` vector to represent the NXPHeader section of the NXS file.
    // &bytes_vec[0 to NXPHeader_size]
    let header_bytes = &bytes_vec[..header_size];
    // convert into the NXPHeader struct
    let header: NXPHeader =
        bincode::deserialize(header_bytes).expect("Failed to deserialize NXPHeader");
    println!("header: {:?}", String::from_utf8_lossy(header_bytes));
    // project content
    let project_content_bytes = &bytes_vec[header_size..];
    println!(
        "debug bytes {:?}",
        String::from_utf8_lossy(project_content_bytes)
    );
    let project_content: NXPContent =
        bincode::deserialize(project_content_bytes).expect("Failed to deserialize project content");
    println!("project_content: {:?}", project_content);
}
