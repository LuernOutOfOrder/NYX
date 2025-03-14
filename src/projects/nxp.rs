/*!
Module to handle the NXP (nyx project) binary format to store projects data.

This module provides the definition of data structures and methods to create and handle the binary format used for storing project data in the NXS format and the NXP format.
It includes functionalities for reading, writing, and manipulating the binary data to ensure efficient storage and retrieval.
*/

use crate::utils;
use sha1::{Digest, Sha1};
use std::fs::File;
use std::io::Write;
use std::time::SystemTime;

/*
NXP file structure
*/
struct NXP {
    header: NXPHeader,
    content: NXPContent,
}

struct NXPHeader {
    magic_number: [u8; 4],
    format_version: [u8; 6],
    project_id: [u8; 11],
    project_size: u32,
}

struct NXPContent {
    pub name: String,
    pub tech: String,
    pub location: String,
    pub repository: String,
    pub github_project: String,
    pub version: String,
    pub todo: String,
}

pub fn create_new_nxp() {
    utils::change_work_dir(&utils::get_nyx_env_var());
    let test_project: NXP = NXP {
        header: NXPHeader {
            magic_number: *b"NXP\0",
            format_version: *b"0.1.0\0",
            project_id: [0u8; 11],
            project_size: 0,
        },
        content: NXPContent {
            name: format!("{}\0", "pro"),
            tech: format!("{}\0", "Rust"),
            location: format!("{}\0", ""),
            repository: format!("{}\0", ""),
            github_project: format!("{}\0", ""),
            version: format!("{}\0", ""),
            todo: format!("{}\0", ""),
        },
    };
    let now = SystemTime::now();
    let mut new_hash = Sha1::new();
    new_hash.update(test_project.content.name + &format!("{:?}", now));
    let hash_result = new_hash.finalize();
    let mut folder_hash = format!("{:#x}", hash_result);
    let (file_hash, _) = folder_hash.split_at(11);
    let header: NXPHeader = NXPHeader {
        magic_number: *b"NXP\0",
        format_version: *b"0.1.0\0",
        project_id: {
            let mut array = [0u8; 11];
            let bytes = file_hash.as_bytes();
            array.copy_from_slice(&bytes[..11]);
            array
        },
        project_size: 0,
    };
    let mut header_buff: Vec<u8> = Vec::new();
    header_buff.extend_from_slice(&header.magic_number);
    header_buff.extend_from_slice(&header.format_version);
    header_buff.extend_from_slice(&header.project_id);
    header_buff.push(0);
    header_buff.push((header.project_size as u32).try_into().unwrap());
    header_buff.push(0);
    println!("nxp_header {:?}", header_buff);
    let content: NXPContent = NXPContent {
        name: format!("{}\0", "pro"),
        tech: format!("{}\0", "Rust"),
        location: format!("{}\0", ""),
        repository: format!("{}\0", ""),
        github_project: format!("{}\0", ""),
        version: format!("{}\0", ""),
        todo: format!("{}\0", ""),
    };
    let mut content_buff: Vec<u8> = Vec::new();
    content_buff.extend_from_slice(content.name.as_bytes());
    content_buff.extend_from_slice(content.tech.as_bytes());
    content_buff.extend_from_slice(content.location.as_bytes());
    content_buff.extend_from_slice(content.repository.as_bytes());
    content_buff.extend_from_slice(content.github_project.as_bytes());
    content_buff.extend_from_slice(content.version.as_bytes());
    content_buff.extend_from_slice(content.todo.as_bytes());
    println!("nxp_content {:?}", content_buff);
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
    lrncore::logs::info_log("Initialized NXS file");
}

pub fn parse_nxp_file(path: &str) {
    utils::change_work_dir(&utils::get_nyx_env_var());
    let file_result: File;
    let file = match File::open(path) {
        Ok(f) => file_result = f,
        Err(e) => {
            lrncore::logs::error_log(&format!("Failed to open nxp file: {}", e));
        }
    };
}
