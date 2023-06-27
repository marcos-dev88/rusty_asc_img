use std::env;

pub mod cli;
pub mod file;
pub mod img_ascii;

fn main() {
    let cli_p = match cli::collect_cli_param::get_params(env::args().collect()) {
        Ok(cli_p) => cli_p,
        Err(e) => {
            panic!("error to get params: {}", e.message)
        }
    };

    let new_img_ascii = img_ascii::new_image_ascii(
        *cli_p.width(),
        *cli_p.heigth(),
        *cli_p.reversed_ascii(),
        *cli_p.colorized(),
        *cli_p.use_block_ascii(),
    );

    if cli_p.path().is_empty() {
        panic!("path is not defined, please define your path using flag '-p' like:\n asc2 -p your-image-path.png\n\t\t")
    }

    match new_img_ascii.gen_art(cli_p.path()) {
        Ok(img_str) => {
            if !cli_p.save_file_path().is_empty() {
                let save_to = file::new(
                    cli_p.save_file_path().to_string(),
                    img_str.as_bytes().to_vec(),
                );

                match save_to.save_to_path() {
                    Ok(_) => {
                        println!("success saved in {}", save_to.path());
                    }
                    Err(e) => {
                        panic!("error to save file: {}", e.message)
                    }
                }
            }
            println!("{}", img_str);
        }
        Err(e) => {
            panic!("error to gen ascii art: {}", e.message)
        }
    };
}
