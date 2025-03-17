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
use std::time::SystemTime;

/*
NXP file structure
*/
struct NXP {
    header: NXPHeader,
    content: NXPContent,
}

#[derive(Debug, Deserialize, Serialize)]
struct NXPHeader {
    magic_number: [u8; 4],
    format_version: [u8; 6],
    project_id: [u8; 11],
    project_size: u32,
    reserved: u32,
}

#[derive(Debug, Deserialize, Serialize)]
struct NXPContent {
    name: Vec<u8>,
    tech: Vec<u8>,
    location: Vec<u8>,
    repository: Vec<u8>,
    github_project: Vec<u8>,
    version: Vec<u8>,
    todo: Vec<u8>,
}

pub fn create_new_nxp() {
    utils::change_work_dir(&utils::get_nyx_env_var());
    // hash
    let now = SystemTime::now();
    let mut new_hash = Sha1::new();
    new_hash.update("prout".to_owned() + &format!("{:?}", now));
    let hash_result = new_hash.finalize();
    let mut folder_hash = format!("{:#x}", hash_result);
    let (file_hash, _) = folder_hash.split_at(11);
    // content
    let content: NXPContent = NXPContent {
        name: b"j".to_vec(),
        tech: b"j".to_vec(),
        location: b"j".to_vec(),
        repository: b"j".to_vec(),
        github_project: b"j".to_vec(),
        version: b"j".to_vec(),
        todo: b"j".to_vec(),
    };
    let mut content_buff = bincode::serialize(&content).expect("Failed to serialize NXPContent");
    content_buff.push(0);
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
    let mut header_buff: Vec<u8> = Vec::new();
    header_buff.extend_from_slice(&header.magic_number);
    header_buff.extend_from_slice(&header.format_version);
    header_buff.extend_from_slice(&header.project_id);
    header_buff.extend_from_slice(&header.project_size.to_le_bytes());
    header_buff.extend_from_slice(&header.reserved.to_le_bytes());
    header_buff.extend_from_slice(b" ");
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
    println!("header: {:?}", header);
    // project content
    let project_content_bytes = &bytes_vec[header_size..];
    println!(
        "debug {:?}",
        std::mem::size_of_val(&project_content_bytes) % 8
    );
    let project_content: NXPContent =
        bincode::deserialize(project_content_bytes).expect("Failed to deserialize project content");
    println!("project_content: {:?}", project_content);
}
