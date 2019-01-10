use crate::file_packer::FilePack;
use cargo_make::types::Task;
use indexmap::IndexMap;
use std::fs::File;
use std::io::prelude::*;
use std::path::{PathBuf};

pub trait FormatRust<T> {
    fn to_rust(&self) -> String;
}

impl FormatRust<Option<Vec<String>>> for Option<Vec<String>> {
    fn to_rust(&self) -> String {
        match self {
            Some(lines) => {
                let mut base = r#"Some(vec!["#.to_string();
                for line in lines {
                    base.push_str(&format!("{}, ", line.to_rust()));
                }
                base.push_str("]),");
                base
            }
            None => "None,".to_string(),
        }
    }
}

impl FormatRust<Option<String>> for Option<String> {
    fn to_rust(&self) -> String {
        match self {
            Some(line) => format!("Some({}),", line.to_rust()),
            None => "None,".to_string(),
        }
    }
}

impl FormatRust<String> for String {
    fn to_rust(&self) -> String {
        format!("{:?}.to_string()", self)
    }
}


pub fn task_from_filename(filename: &str, path: PathBuf) -> Task {
        //println!("{}", filename);
        let mut task = Task::new();
        match filename {
        "Cargo.toml" => {

            task.command = Some("cargo".to_string());
            task.args = Some(vec!["build".to_string()]);
            task.cwd = Some(path.to_string_lossy().into_owned());
            //task.alias
            task
        },
        "Rakefile" => {
        
            task.command = Some("rake".to_string());
            task.cwd = Some(path.to_string_lossy().into_owned());
            //println!("{:?}", task);
            task

        },
        "Makefile" => {
            task.command = Some("make".to_string());
            task.cwd = Some(path.to_string_lossy().into_owned());
            task
        },
        "package.json" => {
            task.command = Some("npm".to_string());
            task.args = Some(vec!["start".to_string()]);
            task.cwd = Some(path.to_string_lossy().into_owned());
            task
        },
        _ => {
            task
        },
    }
}

impl FormatRust<IndexMap<String, Task>> for IndexMap<String, Task> {
    fn to_rust(&self) -> String {
        let mut tasks = r#"
    let mut tasks: IndexMap<String, (FlowInfo, Step)> = IndexMap::new();"#
            .to_string();

        for (name, task) in self {
            let task =   r#"
    let task = Task {
        clear: None,
        description: None,
        category: None,
        disabled: None,
        private: None,
        workspace: None,
        condition: None,
        condition_script: None,
        force: None,
        env: None,
        cwd: $cwd
        alias: None,
        linux_alias: None,
        windows_alias: None,
        mac_alias: None,
        install_crate: None,
        install_crate_args: None,
        install_script: None,
        command: $command
        args: $args
        script: $script
        script_runner: $script_runner
        script_extension: None,
        script_path: None,
        run_task: None,
        dependencies: None,
        toolchain: None,
        linux: None,
        windows: None,
        mac: None
    };
    let step = Step {
        name: "$name".to_string(),
        config: task,
    };
    let flow_info = FlowInfo {
        config: config.clone(),
        task: "$name".to_string(),
        env_info: EnvInfo {
            rust_info: RustInfo::new(),
            crate_info: CrateInfo::new(),
            git_info: GitInfo::new(),
        },
        disable_workspace: false,
        disable_on_error: false,
        cli_arguments: None,
    };
    tasks.insert("$name".to_string(), (flow_info, step));

            "#.to_string()
                .replace("$command", &task.command.to_rust())
                .replace("$args", &task.args.to_rust())
                .replace("$script_runner", &task.script_runner.to_rust())
                .replace("$script", &task.script.to_rust())
                .replace("$cwd", &task.cwd.to_rust())
                //.replace("$env", &task.env.to_rust())
                .replace("$name", &name);

            tasks.push_str(&task);
        }
        //println!("{}", init_map);
        tasks
    }
}

fn run_all_to_code() -> String {
    r#"
    let opt =  Opt::from_args();
    for task_name in opt.entrypoint {
        let (task, step) = &tasks[&task_name];
        //println!("{:?}", task_name);
        run_task(&task, &step);
    }"#
    .to_string()
}

fn all_code(tasks: IndexMap<String, Task>, filepack: &FilePack) -> String {
    let mut top = r#"
extern crate offbin;
extern crate structopt;
extern crate rust_info;

use cargo_make::types::{Step, FlowInfo, Task, Config, ConfigSection, EnvInfo, GitInfo, CrateInfo};
use cargo_make::runner::run_task;
use indexmap::IndexMap;
use offbin::file_packer::FilePack;
use structopt::StructOpt;
use rust_info::types::RustInfo;

#[derive(StructOpt, Debug)]
#[structopt(name = "fantom_installer")]
struct Opt {
    #[structopt(short = "d", long = "debug")]
    debug: bool,

    #[structopt(short = "v", long = "verbose", parse(from_occurrences))]
    version: u8,

    #[structopt(short = "h", long = "help", parse(from_occurrences))]
    help: u8,

    #[structopt(long = "dry-run", parse(from_occurrences))]
    dry_run: u8,

    #[structopt(short = "nc", long = "no-cleanup", parse(from_occurrences))]
    no_cleanup: u8,

    #[structopt(short = "o", long = "output", parse(from_occurrences))]
    output: u8,

    #[structopt(short = "e", long = "entrypoint")]
    entrypoint: Vec<String>,
}

fn main(){

    let config = Config {
        config: ConfigSection::new(),
        env: IndexMap::new(),
        tasks: IndexMap::new(),
    };
"#
        .to_string();

    top.push_str(&filepack.to_rust());
    top.push_str(&tasks.to_rust());
    top.push_str(&run_all_to_code());

    top.push_str(
        r#"
}"#,
    );

    top
}

pub fn generate_rust_file(path: PathBuf, tasks: IndexMap<String, Task>, filepack: &FilePack) {
    let code = all_code(tasks, filepack);
    let mut file = File::create(path.clone()).expect(&format!("file path: {} not valid", path.display()));
    //file.write_all(filepack.to_rust().as_bytes()).expect("could not write to file");
    file.write_all(code.as_bytes())
        .expect("could not write to file");
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn test_save_to_rust_file() {

    //     let mut tasks: IndexMap<String, Task> = IndexMap::new();
    //     tasks.insert("t1".to_string(), Task::new());
    //     tasks.insert("t2".to_string(), Task::new());
    //     generate_rust_file("generated.rs", tasks);
    // }

    #[test]
    fn test_script() {
        let script = Some(vec![
            "#!/usr/bin/env".to_string(),
            "echo \"helloooo\"".to_string(),
        ]);
        println!("{:?}", script);
        println!("{}", script.to_rust());
        assert_eq!(3, 3);
    }

    // #[test]
    // fn test_run_all(){
    //     let mut tasks: IndexMap<String, Task> = IndexMap::new();
    //     tasks.insert("t1".to_string(), Task::new());
    //     tasks.insert("t2".to_string(), Task::new());
    //     let run =  all_code(tasks);
    //     println!("{}", run);
    //     assert_eq!(3, 3);
    // }

    #[test]
    fn test_task() {
        let task = Task {
            clear: None,
            description: None,
            category: None,
            disabled: None,
            private: None,
            workspace: None,
            condition: None,
            condition_script: None,
            force: None,
            env: None,
            cwd: None,
            alias: None,
            linux_alias: None,
            windows_alias: None,
            mac_alias: None,
            install_crate: None,
            install_crate_args: None,
            install_script: None,
            command: None,
            args: None,
            script: Some(vec![
                "#!/usr/bin/env".to_string(),
                "echo \"Hello World\"".to_string(),
            ]),
            script_runner: None,
            script_extension: None,
            script_path: None,
            run_task: None,
            dependencies: None,
            toolchain: None,
            linux: None,
            windows: None,
            mac: None,
        };

        println!("{}", task.to_rust());

        assert_eq!(3, 3);
    }

    #[test]
    fn test_index_map() {
        use cargo_make::types::Task;

        let t1 = Task {
            clear: None,
            description: None,
            category: None,
            disabled: None,
            private: None,
            workspace: None,
            condition: None,
            condition_script: None,
            force: None,
            env: None,
            cwd: None,
            alias: None,
            linux_alias: None,
            windows_alias: None,
            mac_alias: None,
            install_crate: None,
            install_crate_args: None,
            install_script: None,
            command: None,
            args: None,
            script: Some(vec!["echo \"Hello World\"".to_string()]),
            script_runner: None,
            script_extension: None,
            script_path: None,
            run_task: None,
            dependencies: None,
            toolchain: None,
            linux: None,
            windows: None,
            mac: None,
        };

        let t2 = Task {
            clear: None,
            description: None,
            category: None,
            disabled: None,
            private: None,
            workspace: None,
            condition: None,
            condition_script: None,
            force: None,
            env: None,
            cwd: None,
            alias: None,
            linux_alias: None,
            windows_alias: None,
            mac_alias: None,
            install_crate: None,
            install_crate_args: None,
            install_script: None,
            command: None,
            args: None,
            script: Some(vec!["echo \"Hey\"".to_string()]),
            script_runner: None,
            script_extension: None,
            script_path: None,
            run_task: None,
            dependencies: None,
            toolchain: None,
            linux: None,
            windows: None,
            mac: None,
        };

        let mut tasks: IndexMap<String, Task> = IndexMap::new();
        tasks.insert("task1".to_string(), t1);
        tasks.insert("task2".to_string(), t2);

        let a = tasks.to_rust();

        assert_eq!(3, 3);
    }
}
