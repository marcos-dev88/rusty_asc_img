use std::fs::File;
use std::io::Write;
pub mod img_ascii;

fn main() {
    let new_img_ascii = img_ascii::new_image_ascii(75, 35);
    let img_str = new_img_ascii.gen_art("./img/cargo_crab.png");
    let file_name = "./img/test.txt";
    println!("{}", img_str);
    let mut file = File::create(file_name).unwrap();
    writeln!(file, "{}", img_str).unwrap();
}
