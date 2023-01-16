pub mod generate;

#[derive(Debug)]
pub struct ImageASCII{
    width: u32,
    height: u32,
}

pub fn new_image_ascii(w:u32, h:u32) -> ImageASCII { 
    ImageASCII{width: w, height: h}
}
