use std::env;

pub mod img_ascii;
pub mod cli;

fn main() {
    let cli_p = cli::collect_cli_param::get_params(env::args().collect());
    let new_img_ascii = img_ascii::new_image_ascii(*cli_p.width(), *cli_p.heigth(), *cli_p.reversed_ascii());
    let img_str = new_img_ascii.gen_art(cli_p.path());
    // let file_name = "./img/test.txt";
    println!("{}", img_str);
    // let mut file = File::create(file_name).unwrap();
    // writeln!(file, "{}", img_str).unwrap();
}
