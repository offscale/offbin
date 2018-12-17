use std::fs;

#[derive(Debug)]
pub enum Runner {
    CargoMake,
    Make,
    Grunt,
}

pub fn match_runner_to_file(path: &str) -> Option<Runner> {
    if path.contains(".toml") {
        Some(Runner::CargoMake)
    } else {
        None
    }
}

pub fn get_files_in_folder(path: &str) -> Vec<String> {
    let paths = fs::read_dir(path).unwrap();

    let mut file_list = Vec::new();
    for path in paths {
        if let Ok(path) = path {
            file_list.push(path.path().to_string_lossy().into_owned());
        }
    }
    return file_list;
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_get_files_in_folder() {
        let mut path = String::new();
        path.push_str(env!("CARGO_MANIFEST_DIR"));
        path.push_str("/assets");
        let files = get_files_in_folder(&path);

        println!("{:?}", files);
        assert_eq!(2, 2);
    }

    #[test]
    fn test_match_runner_to_file() {
        let path = "hello.toml";

        let runner = match_runner_to_file(path);
        println!("{:?}", runner);

        assert_eq!(2, 2);
    }
}
