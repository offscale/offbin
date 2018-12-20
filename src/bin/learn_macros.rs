extern crate offbin;


use std::env;
use std::fs;
use std::io::Write;
use std::path::Path;
use cargo_make::types::{Task, ExternalConfig};
use cargo_make::scriptengine::invoke;


macro_rules! task  {
    ($a:expr) => {
        let task_string = r#"
        "#
        
        str::replace("hello! $a", "$a", $a) 
    
    };
}



fn main() {
    //println!("{}", hey!("boy"));
}


// fn task_codegen(task: &Task) -> String {
//     let template = r#"
//         let tasks = vec![Task {
//             clear: None,
//             description: None,
//             category: None,
//             disabled: None,
//             private: None,
//             workspace: None,
//             condition: None,
//             condition_script: None,
//             force: None,
//             env: None,
//             cwd: None,
//             alias: None,
//             linux_alias: None,
//             windows_alias: None,
//             mac_alias: None,
//             install_crate: None,
//             install_crate_args: None,
//             install_script: None,
//             command: None,
//             args: None,
//             script: None,
//             script_runner: $script_runner,
//             script_extension: ${wfwefwf},
//             script_path: ${wef},
//             run_task: None,
//             dependencies: None,
//             toolchain: None,
//             linux: None,
//             windows: None,
//             mac: None,
//         }]
//     "#;
//     let task_str = task.replace("$script_runner", task.script_runner.to_string);
//     //let task_str = str::replace(task, )

//     task_str.to_string()
// }