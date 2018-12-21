
use cargo_make::types::Task;
use cargo_make::scriptengine::invoke;
use indexmap::IndexMap;

fn main(){
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
        script: Some(vec!["touch 'testfile.txt'".to_string(), ]),
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
        script: Some(vec!["echo \"Hello World From Unknown\"".to_string(), ]),
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
    for (_, task) in tasks {
        invoke(&task, &Vec::new());
    }
}