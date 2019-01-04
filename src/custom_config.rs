use std::fs;

#[derive(Clone, Debug, Deserialize)]
pub struct Config {
    pub name: String,
    pub description: String,
    pub tasks: Option<Vec<Task>>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Task {
    pub name: String,
    pub command: String,
    pub args: Vec<String>,
}

impl Task {
    pub fn execute(&self) -> Vec<u8> {
        use std::process::Command;

        let output = Command::new(self.command.clone())
            .args(self.args.clone())
            .output()
            .expect("failed to execute process");

        output.stdout
    }
}

impl Config {
    pub fn load_from_file(filename: &str) -> Config {
        let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

        toml::from_str(&contents).unwrap()
    }

    pub fn execute_tasks(&self) {
        match &self.tasks {
            Some(list) => {
                for task in list {
                    task.execute();
                }
            }
            None => (),
        }
    }
}


// pub struct Tool {
//     command: String,
//     args: Vec<String>
// }

// impl Tool {

//     pub fn new(cmd: &str, args: Vec<&str>) -> Self {
//         Tool {
//             command: cmd.to_string(),
//             args: args.iter().map(|s| s.to_string()).collect(),
//         }
//     }

//     pub fn from_filename(filename: &str) -> Self {
        
//         match filename {
//             "Cargo.toml" => Tool::new("cargo", vec!["build"]),
//             "Rakefile" => Tool::new("rake", vec![""]),
//             "Make" => Tool::new("make", vec![""]),
//             "Package.json" => Tool::new("npm", vec!["install"]),
//             _ => Tool::new("", vec![""]),
//         }
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_config_from_string() {
        let toml_str = r#"

        [[tasks]]
        name = "hello_rake"
        command = "rake"
        args = [
            "--rakefile",
            "/Users/rishflab/offbin/assets/Rakefile"
        ]
        "#;

        let decoded: Config = toml::from_str(toml_str).unwrap();
        println!("{:#?}", decoded);
        assert_eq!(2, decoded.tasks.unwrap().first().unwrap().args.len());
    }

    #[test]
    fn test_load_from_file() {
        let mut path = String::new();
        path.push_str(env!("CARGO_MANIFEST_DIR"));
        path.push_str("/assets/offbin.toml");

        let decoded = Config::load_from_file(&path);
        println!("{:#?}", decoded);
        assert_eq!(2, decoded.tasks.unwrap().len());
    }

    #[test]
    fn test_execute_task() {
        let task = Task {
            name: "hello_from_rake".to_string(),
            command: "rake".to_string(),
            args: vec![
                "--rakefile".to_string(),
                "/Users/rishflab/offbin/assets/Rakefile".to_string(),
            ],
        };

        let output = task.execute();

        println!("{:?}", output);

        assert_eq!(2, 2);
    }
}
