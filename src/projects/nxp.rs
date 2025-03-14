/*!
Module to handle the NXP (nyx project) binary format to store projects data.

This module provides the definition of data structures and methods to create and handle the binary format used for storing project data in the NXS format and the NXP format.
It includes functionalities for reading, writing, and manipulating the binary data to ensure efficient storage and retrieval.
*/

use crate::utils;

/*
NXP file structure
*/
struct NXP {
    header: NXPHeader,
}

struct NXPHeader {
    magic_number: [u8; 4],
    format_version: [u8; 6],
    project_id: u8,
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
    let header: NXPHeader = NXPHeader {
        magic_number: *b"NXP\0",
        format_version: *b"0.1.0\0",
        project_id: 0,
        project_size: 0,
    };
    let mut header_buff: Vec<u8> = Vec::new();
    header_buff.extend_from_slice(&header.magic_number);
    header_buff.extend_from_slice(&header.format_version);
    header_buff.push(header.project_id);
    header_buff.push((header.project_size as u32).try_into().unwrap());
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
}
