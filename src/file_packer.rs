use crate::codegen::FormatRust;
use std::collections::HashMap;
use std::fs::{self, DirBuilder, File};
use std::io::Write;
use std::path::PathBuf;
use std::str;

#[derive(Clone, Debug, Serialize)]
pub struct FilePack(pub HashMap<String, Vec<u8>>);

impl FilePack {
    pub fn new() -> FilePack {
        FilePack(HashMap::new())
    }

    pub fn add_file(&mut self, path: &str) {
        let contents = fs::read(path).expect("Something went wrong reading the file");
        self.0.insert(path.to_string(), contents);
    }

    pub fn unpack_files(&self) {
        for (path, contents) in self.0.clone() {
            let pathbuf = PathBuf::from(path);
            {
                let mut clone = pathbuf.clone();
                clone.pop();
                //println!("{:?}", clone.clone());
                let _ = DirBuilder::new().recursive(true).create(clone);
            }
            //println!("{:?}", pathbuf.clone());
            let mut file = File::create(pathbuf).expect("file path not valid");
            file.write_all(&contents).expect("could not write to file");
        }
    }
}

impl FormatRust<Vec<u8>> for Vec<u8> {
    fn to_rust(&self) -> String {
        // r#"

        // "#.to_string()
        format!("{:?}", self)
    }
}

impl FormatRust<FilePack> for FilePack {
    fn to_rust(&self) -> String {
        let mut top = r#"
    let mut filepack = FilePack::new();
        "#
        .to_string();

        for (filename, contents) in &self.0 {
            let template = r#"
    filepack.0.insert("$k".to_string(), vec!$v);
            "#
            .to_string();
            //println!("{}, {}", filename, contents.to_rust());
            let replaced = template.replace("$k", &filename);
            let replaced = replaced.replace("$v", &contents.to_rust());

            top.push_str(&replaced);
        }

        top.push_str(
            r#"
    filepack.unpack_files();
            "#,
        );

        top
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_packed_files_to_rust() {
        let files = vec!["assets/hello.toml", "assets/Makefile"];
        let mut filepack = FilePack::new();

        for filename in files {
            filepack.add_file(&filename);
        }

        //println!("{:?}", filepack);

        println!("{}", filepack.to_rust());
        assert_eq!(4, 3);
    }

    #[test]
    fn test_file_contents_to_rust() {
        let contents = "Wefefwefewf".to_string().into_bytes();

        println!("{}", contents.to_rust());
        assert_eq!(3, 3);
    }

    #[test]
    fn test_unpack_files() {
        let files = vec!["hello.toml", "Makefile"];
        let mut filepack = FilePack::new();

        for filename in files {
            filepack.add_file(&filename);
        }

        filepack.unpack_files();

        assert_eq!(4, 3);
    }
}
