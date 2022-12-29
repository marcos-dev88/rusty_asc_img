use std::fs::File;
use std::io::Write;
pub mod img_ascii;

fn main() {
    let img_str = img_ascii::gen_ascii_art("../../img/cargo_crab.png", 75, 30);
    let file_name = "../../img/test.txt";
    println!("{}", img_str);
    let mut file = File::create(file_name).unwrap();
    writeln!(file, "{}", img_str).unwrap();
}
