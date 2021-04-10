use std::fs::read;

pub fn file_as_bytes(filename: &str) -> Vec<u8> {
    match read(filename) {
        Ok(bytes) => { return bytes },
        Err(e) => { panic!("problem reading: {}\n {}", filename, e)}
    }
}

pub struct ParsedFile {
    file_name: String,
    relative_path: String,
    bytes: Vec<u8>
}

impl ParsedFile {
    pub fn new(name: &str, path: &str) -> ParsedFile {
        let file = ParsedFile::full_path(path, name);
        ParsedFile {
            file_name: String::from(name),
            relative_path: String::from(path),
            bytes: file_as_bytes(&file),
        }
    }
    pub fn full_path(path: &str, name: &str) -> String {
        format!("{}{}", path, name)
    }

    pub fn bytes(&self) -> &Vec<u8> {
        &self.bytes
    }

    pub fn name(&self) -> &str {
        &self.file_name
    }
}