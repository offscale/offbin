extern crate structopt;
extern crate toml;
#[macro_use]
extern crate serde_derive;
extern crate rust_info;
extern crate indexmap;

pub mod cli;
pub mod config;
pub mod custom_config;
pub mod cargo_gen;
pub mod codegen;
pub mod file_packer;