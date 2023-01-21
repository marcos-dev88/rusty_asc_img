pub mod generate;

#[derive(Debug)]
pub struct ImageASCII{
    width: u32,
    height: u32,
    reversed_ascii: bool,
}

pub fn new_image_ascii(w:u32, h:u32, rev: bool) -> ImageASCII { 
    ImageASCII{width: w, height: h, reversed_ascii: rev}
}
