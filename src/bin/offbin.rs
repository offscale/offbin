extern crate offbin;
extern crate indexmap;

use std::env;
use std::fs;
use std::io::Write;
use std::path::Path;
use cargo_make::types::{Task, ExternalConfig};
use cargo_make::scriptengine::invoke;
use offbin::codegen::{script_to_code, main_to_code, imports_to_code};
use indexmap::IndexMap;

use std::error::Error;
use std::io::prelude::*;
use std::fs::File;



fn main() {
    //let config_file = env::var("CONFIG_FILE").unwrap();

    let config_file = "offbin/assets/offbin.toml";

    //let dest_path = Path::new(&config_file).join("hello.rs");
    //let mut f = File::create(&dest_path).unwrap();

    let contents = fs::read_to_string(config_file)
        .expect("Something went wrong reading the file");

   

    let decoded: ExternalConfig = toml::from_str(&contents).unwrap();

    println!("{:?}", decoded.clone().tasks.unwrap());
    let mut a = imports_to_code();
    a.push_str(&format!("let a = {:?}", decoded));
    //let mut a = format!("let a = {:?}", decoded);

    let a = main_to_code(a);
    println!("{:?}", a);

    let path = Path::new("offbin/src/bin/codegen.rs");
    let display = path.display();

    // Open a file in write-only mode, returns `io::Result<File>`
    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}",
                           display,
                           why.description()),
        Ok(file) => file,
    };

    // Write the `LOREM_IPSUM` string to `file`, returns `io::Result<()>`
    match file.write_all(a.as_bytes()) {
        Err(why) => {
            panic!("couldn't write to {}: {}", display,
                                               why.description())
        },
        Ok(_) => println!("successfully wrote to {}", display),
    }

    // {
    //     let task = decoded.tasks.unwrap().get("hello-world-common").unwrap().clone();
    //     invoke(&task, &vec![]);
    // }

    // {
    //     decoded.tasks.unwrap().iter().for_each(|(name, task)|{
    //         println!("{:?}", name);
    //           println!("{:?}", task);
    //     });
    // }


    

    //ExternalConfig::
//    f.write_all(b"
//    pub fn message() -> &'static str {
//        \"Hello, World!\"
//    }
//    ").unwrap();
}


// fn task_to_rust_code(task: &Task) -> String {
    
//     let code  = r#"
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
//             script_runner: ${ewfwef},
//             script_extension: ${wfwefwf},
//             script_path: ${wef},
//             run_task: None,
//             dependencies: None,
//             toolchain: None,
//             linux: None,
//             windows: None,
//             mac: None,
//         }]
//     "
    
 
// }