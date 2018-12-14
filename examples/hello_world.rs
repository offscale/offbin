use std::fs;

fn main() {
    let paths = fs::read_dir("./assets").unwrap();

    for path in paths {
        println!("Name: {}", path.unwrap().path().display())
    }
}

enum Runner {
    CargoMake,
    Make,
    Grunt,
}

fn match_runner_to_file(path: String) -> Option<Runner> {
    if path.contains("*.toml") {
        Some(Runner::CargoMake)
    } else {
        None
    }
}

fn get_files_in_folder() -> Vec<String> {
    let paths = fs::read_dir("./assets").unwrap();

    for path in paths {
        println!("Name: {}", path.unwrap().path().display())
    }
}