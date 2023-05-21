use std::fmt;

pub mod save_file;

type Result<T> = std::result::Result<T, ErrorFile>;

pub struct File {
    path: String,
    content: Vec<u8>,
}

impl File {
    pub fn path(&self) -> &String {
        &self.path
    }

    pub fn content(&self) -> &Vec<u8> {
        &self.content
    }
}

impl File {
    pub fn set_path(&mut self) -> &mut String {
        &mut self.path
    }

    pub fn set_content(&mut self) -> &mut Vec<u8> {
        &mut self.content
    }
}

pub fn new(path: String, content: Vec<u8>) -> File {
    File { path, content }
}

// Custom file Error
#[derive(Debug)]
pub struct ErrorFile {
    pub status: u32,
    pub message: String,
}

impl fmt::Display for ErrorFile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "error in file")
    }
}

pub fn new_err(status: u32, message: String) -> ErrorFile {
    ErrorFile { status, message }
}
