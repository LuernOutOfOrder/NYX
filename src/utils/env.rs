/*
This module provides utility functions to manage environment variables.
It includes functions to retrieve specific environment variables and handle errors gracefully.
*/

use std::env;

pub fn get_nyx_env_var() -> String {
    let env_var = "NYX";
    match env::var(env_var) {
        Ok(v) => return v,
        Err(e) => panic!("${} is not set ({})", env_var, e),
    }
}
