extern crate offbin;
extern crate indexmap;

use std::env;
use std::fs;
use std::io::Write;
use std::path::Path;
use cargo_make::types::{Task, ExternalConfig};
use cargo_make::scriptengine::invoke;
use indexmap::IndexMap;

use std::error::Error;
use std::io::prelude::*;
use std::fs::File;
use structopt::StructOpt;
use offbin::codegen::generate_rust_file;
use offbin::custom_config;

#[derive(StructOpt, Debug)]
#[structopt(name = "basic")]
struct Opt {
   
    #[structopt(short = "d", long = "debug")]
    debug: bool,
   
    #[structopt(short = "v", long = "verbose", parse(from_occurrences))]
    version: u8,

    #[structopt(short = "h", long = "help", parse(from_occurrences))]
    help: u8,

    #[structopt(short = "h", long = "help", parse(from_occurrences))]
    pipe: u8,

    #[structopt(short = "d", long = "dry-run", parse(from_occurrences))]
    dry_run: u8,

    #[structopt(short = "nc", long = "no-cleanup", parse(from_occurrences))]
    no_cleanup: u8,

    #[structopt(short = "o", long = "output", parse(from_occurrences))]
    output: u8,
    
    #[structopt(short = "e", long = "entrypoint")]
    entrypoint: String,
}

fn main() {
    //let config_file = env::var("CONFIG_FILE").unwrap();

    let config_file = "assets/offbin.toml";

    //let dest_path = Path::new(&config_file).join("hello.rs");
    //let mut f = File::create(&dest_path).unwrap();

    let contents = fs::read_to_string(config_file)
        .expect("Something went wrong reading the file");

   
    let decoded: ExternalConfig = toml::from_str(&contents).unwrap();

    println!("{:?}", decoded.clone().tasks.unwrap());

    generate_rust_file("src/bin/generated.rs", decoded.tasks.unwrap());

    let task = custom_config::Task {
        name: "build_generated".to_string(),
        command: "cargo".to_string(),
        args: vec!["build".to_string()],
    };

    let output = task.execute();

    
   

}
