use std::fmt;

pub mod generate;

type Result<T> = std::result::Result<T, ErrorGenImage>;

#[derive(Debug)]
pub struct ImageASCII {
    width: u32,
    heigth: u32,
    reversed_ascii: bool,
    colorized: bool,
    block_char: bool,
}

pub fn new_image_ascii(
    width: u32,
    heigth: u32,
    reversed_ascii: bool,
    colorized: bool,
    block_char: bool,
) -> ImageASCII {
    ImageASCII {
        width,
        heigth,
        reversed_ascii,
        colorized,
        block_char,
    }
}

// Custom file Error
#[derive(Debug)]
pub struct ErrorGenImage {
    pub status: u32,
    pub message: String,
}

impl fmt::Display for ErrorGenImage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "error in gen image ascii")
    }
}

pub fn new_err(status: u32, message: String) -> ErrorGenImage {
    ErrorGenImage { status, message }
}
