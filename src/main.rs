use std::env;

pub mod cli;
pub mod img_ascii;

fn main() {
    let cli_p = match cli::collect_cli_param::get_params(env::args().collect()) {
        Ok(cli_p) => cli_p,
        Err(e) => {
            panic!("error to get params: {}", e)
        }
    };

    let new_img_ascii = img_ascii::new_image_ascii(
        *cli_p.width(),
        *cli_p.heigth(),
        *cli_p.reversed_ascii(),
        *cli_p.colorized(),
        *cli_p.use_block_ascii(),
    );

    match new_img_ascii.gen_art(cli_p.path()) {
        Ok(img_str) => {
            println!("{}", img_str);
        }
        Err(e) => {
            panic!("error to gen ascii art: {}", e)
        }
    };
    // let file_name = "./img/test.txt";
    // let mut file = File::create(file_name).unwrap();
    // writeln!(file, "{}", img_str).unwrap();
}
