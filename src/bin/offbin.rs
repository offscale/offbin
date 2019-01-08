extern crate indexmap;
extern crate offbin;

use cargo_make::types::ExternalConfig;
use offbin::codegen::generate_rust_file;
use std::fs::{self, DirBuilder};
use structopt::StructOpt;
//use offbin::custom_config;
use offbin::cargo_gen::CargoToml;
use offbin::file_packer::FilePack;
use offbin::codegen::task_from_filename;

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

    #[structopt(long = "dry-run", parse(from_occurrences))]
    dry_run: u8,

    #[structopt(short = "nc", long = "no-cleanup", parse(from_occurrences))]
    no_cleanup: u8,

    #[structopt(short = "o", long = "output", parse(from_occurrences))]
    output: u8,

    #[structopt(short = "e", long = "entrypoint")]
    entrypoint: String,
}

fn main() {
    let target_directory = "assets/";
    let config_file = "assets/offbin.toml";
    let generated_bin = "fantom_installer/src";

    let contents = fs::read_to_string(config_file).expect("Something went wrong reading the file");

    let decoded: ExternalConfig = toml::from_str(&contents).unwrap();

    let mut tasks = decoded.clone().tasks.unwrap();

    println!("{:?}", decoded.clone().tasks.unwrap());

    let mut filepack = FilePack::new();

    let paths = fs::read_dir(target_directory).unwrap();

    for path in paths {

        let mut filepath = path.unwrap().path();
        
        {
            let clone = filepath.clone();
            filepack.add_file(clone.to_str().unwrap());
        }

        //println!("Name: {}", path.unwrap().path().display());
        let filename = filepath.file_name().unwrap().to_str().unwrap().to_string();
        //println!("Name: {}", filename.clone());
        filepath.pop();
        let task = task_from_filename(&filename, filepath.clone());


        tasks.insert(filename.to_string(), task);

        //println!("Name: {}", path.unwrap().path().display());
    }

    DirBuilder::new()
        .recursive(true)
        .create(generated_bin)
        .unwrap();

    CargoToml::new("fantom_installer").to_file("fantom_installer/Cargo.toml".to_string());

    generate_rust_file(
        "fantom_installer/src/main.rs",
        tasks,
        &filepack,
    );

    // let task = custom_config::Task {
    //     name: "build_generated".to_string(),
    //     command: "cargo".to_string(),
    //     args: vec!["build".to_string()],
    // };

    //    task.execute();
}
