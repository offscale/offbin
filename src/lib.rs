extern crate structopt;
extern crate toml;
#[macro_use]
extern crate serde_derive;
extern crate indexmap;
extern crate rust_info;

pub mod cargo_gen;
pub mod cli;
pub mod codegen;
pub mod custom_config;
pub mod file_packer;
