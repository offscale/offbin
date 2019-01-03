use cargo_make::scriptengine::invoke;
use cargo_make::types::Task;
use indexmap::IndexMap;

fn main() {
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
        script: Some(vec!["echo 'testfile.txt'".to_string()]),
        script_runner: Some("@shell".to_string()),
        script_extension: None,
        script_path: None,
        run_task: None,
        dependencies: None,
        toolchain: None,
        linux: None,
        windows: None,
        mac: None,
    };

    // let mut task = Task::new();
    // task.script_runner = Some("@shell".to_string());
    // task.script = Some(vec!["echo test".to_string()]);

    let output = invoke(&task, &vec![]);

    println!("{:?}", output);

    //invoke(&task, &Vec::new());
}
