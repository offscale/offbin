use std::fs;

// fn main() {
//     let paths = fs::read_dir("./assets").unwrap();

//     for path in paths {
//         println!("Name: {}", path.unwrap().path().display())
//     }
// }

pub enum Runner {
    CargoMake,
    Make,
    Grunt,
}

pub fn match_runner_to_file(path: String) -> Option<Runner> {
    if path.contains("*.toml") {
        Some(Runner::CargoMake)
    } else {
        None
    }
}

pub fn get_files_in_folder(path: String) -> Vec<String> {
    let files = fs::read_dir("String").unwrap();

    let mut file_list = Vec::new(); 
    for path in paths {
        file_list.push(path);
        //println!("Name: {}", path.unwrap().path().display())
    }
    return file_list
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(1, 2), 3);
    }

    #[test]
    fn test_bad_add() {
        // This assert would fire and test will fail.
        // Please note, that private functions can be tested too!
        assert_eq!(bad_add(1, 2), 3);
    }
}

