use cargo_make::types::Task;
use indexmap::IndexMap;
use std::fs::File;
use std::io::prelude::*;

pub trait FormatRust<T> {
    fn to_rust(&self) -> String;
}

impl FormatRust<Option<Vec<String>>> for Option<Vec<String>> {
    fn to_rust (&self) -> String {
        match self {
            Some(lines) => {
                let mut base = 
r#"Some(vec!["#.to_string();  
                for line in lines {      
                    base.push_str(&format!("{}, ", line.to_rust()));   
                }
                base.push_str("]),");
                base
            },
            None => {
                "None,".to_string()
            }
        }      
    }
}

impl FormatRust<Option<String>> for Option<String> {
    fn to_rust(&self) -> String {
        match self {
            Some(line) => {
                format!("Some({}),", line.to_rust())
            },
            None => {
                "None,".to_string()
            }
        }
    }
}

impl FormatRust<String> for String {
    fn to_rust(&self) -> String {
        format!("{:?}.to_string()", self)
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
        script: $a"#.to_string();
        let script = script.replace("$a", &self.script.to_rust());

        let script_runner = r#"
        script_runner: $a"#.to_string();
        let script_runner = script_runner.replace("$a", &self.script_runner.to_rust());

        let bottom = r#"
        script_extension: None, 
        script_path: None, 
        run_task: None, 
        dependencies: None, 
        toolchain: None, 
        linux: None, 
        windows: None, 
        mac: None
    }"#.to_string();

    top.push_str(&script);
    top.push_str(&script_runner);
    top.push_str(&bottom);
    top
    }
}

impl FormatRust<IndexMap<String, Task>> for IndexMap<String, Task> {
    fn to_rust(&self) -> String {
        let mut task_str = r#"
    let mut tasks: IndexMap<String, Task> = IndexMap::new();"#.to_string();

        let template = r#"
    tasks.insert("$k".to_string(), $v);"#.to_string();
        for (name, task) in self {
            println!("{:?}, {:?}", name, task);
            let replaced = template.replace("$k", &name);
            let replaced = replaced.replace("$v", &task.to_rust());
            task_str.push_str(&replaced);
        }
        println!("{}", task_str);
        task_str
    }
}

fn run_all_to_code(tasks: IndexMap<String, Task>) -> String {
    r#"
    for (_, task) in tasks {
        invoke(&task, &Vec::new());
    }"#.to_string()
}

fn all_code(tasks: IndexMap<String, Task>) -> String {
    let mut top = r#"
use cargo_make::types::Task;
use cargo_make::scriptengine::invoke;
use indexmap::IndexMap;

fn main(){"#.to_string();

    top.push_str(&tasks.to_rust());
    top.push_str(&run_all_to_code(tasks));

    top.push_str(r#"
}"#);

    top
}

pub fn generate_rust_file(path: &str, tasks: IndexMap<String, Task>) {
    let code =  all_code(tasks);
    let mut file = File::create(path).expect("file path not valid");
    file.write_all(code.as_bytes()).expect("could not write to file");    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_save_to_rust_file() {

        let mut tasks: IndexMap<String, Task> = IndexMap::new();
        tasks.insert("t1".to_string(), Task::new());
        tasks.insert("t2".to_string(), Task::new());
        generate_rust_file("generated.rs", tasks);
    }


    #[test]
    fn test_script() {
        
        let script = Some(vec!["#!/usr/bin/env".to_string(), "echo \"hellooo\"".to_string()],);
        println!("{:?}", script);
        println!("{}", script.to_rust());
        assert_eq!(3, 3);
    }

    #[test]
    fn test_run_all(){
        let mut tasks: IndexMap<String, Task> = IndexMap::new();
        tasks.insert("t1".to_string(), Task::new());
        tasks.insert("t2".to_string(), Task::new());
        let run =  all_code(tasks);
        println!("{}", run);
        assert_eq!(3, 3);
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
                    "#!/usr/bin/env".to_string(),
                    "echo \"Hello World\"".to_string(),
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

        let mut tasks: IndexMap<String, Task> =  IndexMap::new();
        tasks.insert("task1".to_string(), t1);
        tasks.insert("task2".to_string(), t2);

        let a = tasks.to_rust();
       
        assert_eq!(3,3);
    }     
}
