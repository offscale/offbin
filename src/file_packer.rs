use std::collections::HashMap;
use std::env;
use std::fs;
use std::io::Write;
use std::path::Path;
use crate::codegen::FormatRust;
use std::fs::File;

struct FilePack(HashMap<String, Vec<u8>>);

impl FilePack {

    pub fn new() -> FilePack {
        FilePack(HashMap::new())
    }

    pub fn add_file(&mut self, path: &str) {
        let contents = fs::read(path)
            .expect("Something went wrong reading the file");
        self.0.insert(path.to_string(), contents);
    }

    pub fn unpack_file(&self) {
        for (filename, contents) in self.0.clone() {
            let mut file = File::create(filename).expect("file path not valid");
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
        let mut files = Vec::new();
        for (filename, _) in self.0.clone(){
            files.push(filename);
        };

        r#"
    let mut filepack =  Filepack::new();
    for (filename, contents) in filepack.0 {
        filepack.insert(filename, contents);        
    }"#.to_string()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_packed_files_to_rust() {
        let files =  vec!["assets/hello.toml", "assets/Makefile"];
        let mut filepack =  FilePack::new();
        for filename in files {
            filepack.add_file(&filename);      
        }
        println!("{}", filepack.to_rust());
        assert_eq!(4,3);

    }

     #[test]
    fn test_file_contents_to_rust() {
        let contents = "Wefefwefewf".into_bytes();

        
        println!("{}", contents.to_rust());
        assert_eq!(4,3);

    }
}