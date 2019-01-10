use cargo_make::types::{Task, Step, Config, ConfigSection, FlowInfo, ExternalConfig, EnvInfo, CrateInfo, GitInfo};
use cargo_make::runner::run_task;
use offbin::codegen::generate_rust_file;
use std::fs::{self, DirBuilder};
use structopt::StructOpt;
use offbin::cargo_gen::CargoToml;
use offbin::file_packer::FilePack;
use offbin::codegen::task_from_filename;
use indexmap::IndexMap;
use rust_info::types::RustInfo;
use std::env;
use walkdir::WalkDir;
use std::path::PathBuf;

#[derive(StructOpt, Debug)]
#[structopt(name = "offbin")]
struct Opt {
    #[structopt(short = "d", long = "debug")]
    debug: bool,

    #[structopt(short = "v", long = "verbose", parse(from_occurrences))]
    version: u8,

    #[structopt(short = "h", long = "help", parse(from_occurrences))]
    pipe: u8,

    #[structopt(long = "dry-run", parse(from_occurrences))]
    dry_run: u8,

    #[structopt(short = "nc", long = "no-cleanup", parse(from_occurrences))]
    no_cleanup: u8,

    #[structopt(short = "o", long = "output")]
    output: String,

    #[structopt(short = "i", long = "input")]
    input: String,


//    #[structopt(short = "e", long = "entrypoint")]
//    entrypoint: String,
}

fn main() {

    let config_file = "offbin.toml";

    let opt =  Opt::from_args();

    let output_dir: &str = &opt.output;

    let input_dir: &str = &opt.input;

    //println!("{}", generated_bin);

    let contents = fs::read_to_string(config_file).expect(&format!("Something went wrong reading config file: {}", config_file));

    let decoded: ExternalConfig = toml::from_str(&contents).unwrap();

    let mut tasks = decoded.clone().tasks.unwrap();

    //println!("{:?}", decoded.clone().tasks.unwrap());

    let filepack = FilePack::new();

    //let cwd = env::current_dir().expect(&format!("couldnt find cwd at: {:?}", env::current_dir()));

    //let paths = fs::read_dir(cwd.clone()).unwrap();


    for entry in WalkDir::new(input_dir) {
        match entry {
            Ok(dir_entry) => {
                let metadata = dir_entry.metadata().expect("Could not retreive metadata");
                if metadata.is_file() {
                    println!("{}", dir_entry.path().display());
                    let file_name =  dir_entry.path().file_name().expect("Could not extract file name").to_str().unwrap();
                    let task = task_from_filename(file_name, dir_entry.path().clone().to_path_buf());
                    tasks.insert(file_name.to_string(), task);
                    println!("{:?}", file_name);

                }
            },
            Err(e) => {
                eprintln!("Problem walking directory: {}", e);
            }
        }

    }

    let output_src_path: PathBuf = [output_dir, "src"].iter().collect();
    let output_main_path: PathBuf = [output_dir, "src", "main.rs"].iter().collect();
    let output_cargo_path: PathBuf = [output_dir, "Cargo.toml"].iter().collect();

    DirBuilder::new()
        .recursive(true)
        .create(output_src_path)
        .expect("could not create output directory");


    CargoToml::new(output_dir).to_file(output_cargo_path);

    generate_rust_file(
        output_main_path,
        tasks,
        &filepack,
    );

    let mut task = Task::new();
    task.args = Some(vec!["build".to_string()]);
    task.command = Some("cargo".to_string());
    task.cwd = Some(output_dir.to_string());

    let config = Config {
        config: ConfigSection::new(),
        env: IndexMap::new(),
        tasks: IndexMap::new(),
    };

    let step = Step {
        name: "build".to_string(),
        config: task,
    };
    let flow_info = FlowInfo {
        config: config.clone(),
        task: "build".to_string(),
        env_info: EnvInfo {
            rust_info: RustInfo::new(),
            crate_info: CrateInfo::new(),
            git_info: GitInfo::new(),
        },
        disable_workspace: false,
        disable_on_error: false,
        cli_arguments: None,
    };

    run_task(&flow_info, &step);
}
