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

pub fn create_new_nxp(project: NXP) {
    utils::change_work_dir(&utils::get_nyx_env_var());
    let header: NXPHeader = NXPHeader {
        magic_number: *b"NXP\0",
        format_version: *b"0.1.0\0",
        project_id: (),
        project_size: (),
    };
}
