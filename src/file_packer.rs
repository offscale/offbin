use std::collections::HashMap;
use std::env;
use std::fs;
use std::io::Write;
use std::path::Path;
use crate::codegen::FormatRust;
use std::fs::File;
use std::str;

#[derive(Clone, Debug, Serialize)]
struct FilePack(HashMap<String, Vec<u8>>);


impl FilePack {

    pub fn new() -> FilePack {
        FilePack(HashMap::new())
    }

    pub fn add_file(&mut self, path: &str) {
        let contents = fs::read(path)
            .expect("Something went wrong reading the file");
        //println!("{}", str::from_utf8(&contents).unwrap());
        self.0.insert(path.to_string(), contents);
    }

    pub fn unpack_files(&self) {
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

        let mut top =  r#"
            let filepack = Filepack::new();
        "#.to_string();

       

        for (filename, contents) in &self.0 {
            
            let mut template = r#"
                filepack.insert($k, $v);
            "#.to_string();
            //println!("{}, {}", filename, contents.to_rust());
            let mut replaced = template.replace("$k", &filename);
            let mut replaced = replaced.replace("$v", &contents.to_rust());
            top.push_str(&replaced);

        }

        //println!("{}", top);

        top

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
        
        let files =  vec!["hello.toml", "Makefile"];
        let mut filepack =  FilePack::new();

        for filename in files {
            filepack.add_file(&filename);      
        }

        filepack.unpack_files();
    
        assert_eq!(4, 3);

    }
}