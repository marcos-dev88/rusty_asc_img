pub mod generate;

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
