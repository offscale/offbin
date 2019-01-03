use cargo_make::types::{Task, ExternalConfig};
use crate::codegen::{generate_rust_file};
use std::env;
use std::fs;
use std::io::Write;
use std::path::Path;
use cargo_make::scriptengine::invoke;
use indexmap::IndexMap;

    //let config_file = env::var("CONFIG_FILE").unwrap();

// pub fn generate_binary_from_config(generated_file: &str, config_file: &str){

//     let contents = fs::read_to_string(config_file)
//         .expect("Something went wrong reading the file");

//     let decoded: ExternalConfig = toml::from_str(&contents).unwrap();

//     let tasks = decoded.tasks.unwrap();
    
//     generate_rust_file(generated_file, tasks);

// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_config_to_binary() {
//         let config_file = "assets/offbin.toml";
//         let generated_file = "generated.rs";
//         generate_binary_from_config(generated_file, config_file);
//         assert_eq!(3, 3);
//     }
// }