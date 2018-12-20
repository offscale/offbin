use cargo_make::types::Task;


// pub fn index_map_to_code(tasks: IndexMap<String, Task>) -> String {
//     let mut task_str = "let mut tasks = Indexmap::new();";

//     let template = "tasks.insert($k, $v)";

//     //tasks.insert(, value: V)
//     for (name, task) in tasks {
//         let replaced = template.replace("$k", &name);
//         let replaced = template.replace("$v", &format!("{:?}", task));
//         task_str.push_str(replaced.to_string);
//     }

//     task_str.to_string()
// }

pub trait FormatRust<T> {
    fn to_rust(&self) -> String;
}

impl FormatRust<Vec<String>> for Vec<String> {
    fn to_rust (&self) -> String {
        let mut base = "vec![".to_string();
    
        for line in script {
            base.push_str(&format!("\"{}\", ", line));
        }

        base.push_str("];");
        base
    }
} 

impl FormatRust<Task> for Task {
    fn to_rust (&self) -> String {

        let mut top = 
r#"Task { 
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
    command: None, args: None,"#.to_string();

        let script = r#"
    script: Some(["echo \"Hello World\""]),"#.to_string();

        let bottom = r#"
    script_runner: None, 
    script_extension: None, 
    script_path: None, 
    run_task: None, 
    dependencies: None, 
    toolchain: None, 
    linux: None, 
    windows: None, 
    mac: None
};"#.to_string();

    top.push_str(&script);
    top.push_str(&bottom);

    top
   
    }
}
// pub fn task_to_code(task: Task) -> String {




//     let replacement = script_to_code(task.script.unwrap());

//     let pattern = "script: Some([*])"

//     task_str.replace()

//     task_str.to_string()
// }

pub fn script_to_code(script: Vec<String>) -> String {
    let mut base = "vec!["
        .to_string();
    
    for line in script {
        base.push_str(&format!("\"{}\", ", line));
    }

    base.push_str("];");
    base
}

pub fn main_to_code(mut other: String) -> String {
    let main  = r#"
        fn main(){
            println!("{:?}", a);
        }
    "#;

    other.push_str(main);
    other
}

pub fn imports_to_code() -> String {
    let imports = r#"
        use cargo_make::types::{Task, ExternalConfig};
        use cargo_make::scriptengine::invoke;
    "#;
    imports.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_script() {
        
        let script = vec!["#!/usr/bin/env".to_string(), "echo 'hellooo'".to_string()];
        println!("{:?}", script);
        println!("{}", script_to_code(script));
        assert_eq!(4,3 );
    }

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
            script: Some(
                vec![
                    "echo \"Hello World\"".to_string()
                ]
            ),
            script_runner: None,
            script_extension: None,
            script_path: None,
            run_task: None,
            dependencies: None,
            toolchain: None,
            linux: None,
            windows: None,
            mac: None
        };

        println!("{}", task.to_rust());

        assert_eq!(4,3 );



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
            script: Some(
                vec![
                    "echo \"Hello World\"".to_string()
                ]
            ),
            script_runner: None,
            script_extension: None,
            script_path: None,
            run_task: None,
            dependencies: None,
            toolchain: None,
            linux: None,
            windows: None,
            mac: None
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
            script: Some(
                vec![
                    "echo \"Hey\"".to_string()
                ]
            ),
            script_runner: None,
            script_extension: None,
            script_path: None,
            run_task: None,
            dependencies: None,
            toolchain: None,
            linux: None,
            windows: None,
            mac: None
        };
    }     
}
