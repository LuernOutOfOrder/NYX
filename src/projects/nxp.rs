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
    println!("npx_header {:?}", header_buff);
}
