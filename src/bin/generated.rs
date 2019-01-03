extern crate offbin;

use cargo_make::types::Task;
use cargo_make::scriptengine::invoke;
use indexmap::IndexMap;
use offbin::file_packer::FilePack;

fn main(){
    let mut filepack = FilePack::new();
        
    filepack.0.insert("assets/Gruntfile".to_string(), vec![]);
            
    filepack.0.insert("assets/offbin.toml".to_string(), vec![91, 99, 111, 110, 102, 105, 103, 93, 10, 115, 107, 105, 112, 95, 99, 111, 114, 101, 95, 116, 97, 115, 107, 115, 32, 61, 32, 116, 114, 117, 101, 10, 10, 91, 116, 97, 115, 107, 115, 46, 100, 101, 102, 97, 117, 108, 116, 93, 10, 100, 101, 112, 101, 110, 100, 101, 110, 99, 105, 101, 115, 32, 61, 32, 91, 34, 104, 101, 108, 108, 111, 45, 119, 111, 114, 108, 100, 45, 99, 111, 109, 109, 111, 110, 34, 44, 32, 34, 104, 101, 108, 108, 111, 45, 119, 111, 114, 108, 100, 34, 93, 10, 10, 91, 116, 97, 115, 107, 115, 46, 104, 101, 108, 108, 111, 45, 119, 111, 114, 108, 100, 45, 99, 111, 109, 109, 111, 110, 93, 10, 115, 99, 114, 105, 112, 116, 95, 114, 117, 110, 110, 101, 114, 32, 61, 32, 34, 64, 115, 104, 101, 108, 108, 34, 10, 115, 99, 114, 105, 112, 116, 32, 61, 32, 91, 10, 32, 32, 32, 32, 34, 116, 111, 117, 99, 104, 32, 39, 116, 101, 115, 116, 102, 105, 108, 101, 46, 116, 120, 116, 39, 34, 10, 93, 10, 10, 91, 116, 97, 115, 107, 115, 46, 104, 101, 108, 108, 111, 45, 119, 111, 114, 108, 100, 93, 10, 115, 99, 114, 105, 112, 116, 95, 114, 117, 110, 110, 101, 114, 32, 61, 32, 34, 64, 115, 104, 101, 108, 108, 34, 10, 115, 99, 114, 105, 112, 116, 32, 61, 32, 91, 10, 32, 32, 32, 32, 34, 101, 99, 104, 111, 32, 39, 72, 101, 108, 108, 111, 32, 87, 111, 114, 108, 100, 32, 70, 114, 111, 109, 32, 85, 110, 107, 110, 111, 119, 110, 39, 34, 10, 93, 10, 10, 91, 116, 97, 115, 107, 115, 46, 104, 101, 108, 108, 111, 45, 119, 111, 114, 108, 100, 46, 108, 105, 110, 117, 120, 93, 10, 115, 99, 114, 105, 112, 116, 95, 114, 117, 110, 110, 101, 114, 32, 61, 32, 34, 64, 115, 104, 101, 108, 108, 34, 10, 115, 99, 114, 105, 112, 116, 32, 61, 32, 91, 10, 32, 32, 32, 32, 34, 101, 99, 104, 111, 32, 39, 72, 101, 108, 108, 111, 32, 87, 111, 114, 108, 100, 32, 70, 114, 111, 109, 32, 76, 105, 110, 117, 120, 39, 34, 10, 93, 10, 10]);
            
    filepack.0.insert("assets/cargo.toml".to_string(), vec![91, 112, 97, 99, 107, 97, 103, 101, 93, 10, 110, 97, 109, 101, 32, 61, 32, 34, 102, 97, 110, 116, 111, 109, 45, 105, 110, 115, 116, 97, 108, 108, 101, 114, 34, 10, 118, 101, 114, 115, 105, 111, 110, 32, 61, 32, 34, 48, 46, 49, 46, 48, 34, 10, 97, 117, 116, 104, 111, 114, 115, 32, 61, 32, 91, 34, 114, 105, 115, 104, 102, 108, 97, 98, 32, 60, 114, 105, 115, 104, 102, 108, 97, 98, 64, 104, 111, 116, 109, 97, 105, 108, 46, 99, 111, 109, 62, 34, 93, 10, 101, 100, 105, 116, 105, 111, 110, 32, 61, 32, 34, 50, 48, 49, 56, 34, 10, 10, 91, 100, 101, 112, 101, 110, 100, 101, 110, 99, 105, 101, 115, 93, 10, 104, 121, 112, 101, 114, 32, 61, 32, 34, 42, 34, 10, 10]);
            
    filepack.0.insert("assets/Rakefile".to_string(), vec![116, 97, 115, 107, 32, 58, 100, 101, 102, 97, 117, 108, 116, 32, 100, 111, 10, 32, 32, 112, 117, 116, 115, 32, 34, 72, 101, 108, 108, 111, 32, 102, 114, 111, 109, 32, 82, 97, 107, 101, 33, 34, 10, 32, 32, 102, 111, 111, 46, 116, 120, 116, 10, 101, 110, 100, 10, 10, 102, 105, 108, 101, 32, 39, 102, 111, 111, 46, 116, 120, 116, 39, 32, 100, 111, 10, 32, 32, 116, 111, 117, 99, 104, 32, 39, 102, 111, 111, 46, 116, 120, 116, 39, 10, 101, 110, 100]);
            
    filepack.0.insert("assets/Makefile".to_string(), vec![104, 101, 108, 108, 111, 58, 32, 59, 32, 64, 101, 99, 104, 111, 32, 34, 72, 101, 108, 108, 111, 32, 102, 114, 111, 109, 32, 77, 97, 107, 101, 33, 34]);
            
    filepack.0.insert("assets/hello.toml".to_string(), vec![91, 116, 97, 115, 107, 115, 46, 65, 93, 10, 100, 101, 112, 101, 110, 100, 101, 110, 99, 105, 101, 115, 32, 61, 32, 91, 34, 66, 34, 44, 32, 34, 67, 34, 93, 10, 10, 91, 116, 97, 115, 107, 115, 46, 66, 93, 10, 100, 101, 112, 101, 110, 100, 101, 110, 99, 105, 101, 115, 32, 61, 32, 91, 34, 68, 34, 93, 10, 10, 91, 116, 97, 115, 107, 115, 46, 67, 93, 10, 100, 101, 112, 101, 110, 100, 101, 110, 99, 105, 101, 115, 32, 61, 32, 91, 34, 68, 34, 93, 10, 10, 91, 116, 97, 115, 107, 115, 46, 68, 93, 10, 115, 99, 114, 105, 112, 116, 32, 61, 32, 91, 10, 32, 32, 32, 32, 34, 101, 99, 104, 111, 32, 104, 101, 108, 108, 111, 34, 10, 93]);
            
    filepack.unpack_files();
            
    let mut tasks: IndexMap<String, Task> = IndexMap::new();
    tasks.insert("default".to_string(), Task { 
        clear: None,
        description: None,
        category: None,
        disabled: None,
        private: None,
        workspace: None,
        condition: None,
        condition_script: None,
        force: None, env: None,
        cwd: None, alias: None, 
        linux_alias: None, 
        windows_alias: None, 
        mac_alias: None, 
        install_crate: None, 
        install_crate_args: None, 
        install_script: None, 
        command: None, args: None,
        script: None,
        script_runner: None,
        script_extension: None, 
        script_path: None, 
        run_task: None, 
        dependencies: None, 
        toolchain: None, 
        linux: None, 
        windows: None, 
        mac: None
    });
    tasks.insert("hello-world-common".to_string(), Task { 
        clear: None,
        description: None,
        category: None,
        disabled: None,
        private: None,
        workspace: None,
        condition: None,
        condition_script: None,
        force: None, env: None,
        cwd: None, alias: None, 
        linux_alias: None, 
        windows_alias: None, 
        mac_alias: None, 
        install_crate: None, 
        install_crate_args: None, 
        install_script: None, 
        command: None, args: None,
        script: Some(vec!["touch \'testfile.txt\'".to_string(), ]),
        script_runner: Some("@shell".to_string()),
        script_extension: None, 
        script_path: None, 
        run_task: None, 
        dependencies: None, 
        toolchain: None, 
        linux: None, 
        windows: None, 
        mac: None
    });
    tasks.insert("hello-world".to_string(), Task { 
        clear: None,
        description: None,
        category: None,
        disabled: None,
        private: None,
        workspace: None,
        condition: None,
        condition_script: None,
        force: None, env: None,
        cwd: None, alias: None, 
        linux_alias: None, 
        windows_alias: None, 
        mac_alias: None, 
        install_crate: None, 
        install_crate_args: None, 
        install_script: None, 
        command: None, args: None,
        script: Some(vec!["echo \'Hello World From Unknown\'".to_string(), ]),
        script_runner: Some("@shell".to_string()),
        script_extension: None, 
        script_path: None, 
        run_task: None, 
        dependencies: None, 
        toolchain: None, 
        linux: None, 
        windows: None, 
        mac: None
    });
    for (_, task) in tasks {
        invoke(&task, &Vec::new());
    }
}