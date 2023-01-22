use crate::img_ascii::ImageASCII;
use std::format;

const ASCII_LIST_LENGTH: usize = 15;
const ASCII_LIST: [&str; ASCII_LIST_LENGTH] = [
    " ", ".", ",", "-", "~", "+", "=", "7", "8", "9", "$", "W", "#", "@", "Ñ",
];
const RGB_8_BYTE_VAL_FLOAT: f32 = 255.0;
impl ImageASCII { 
     pub fn gen_art(&self, path: &str) -> String {
        let mut ascii_l: [&str; ASCII_LIST_LENGTH] = ASCII_LIST;

        if self.reversed_ascii {
            ascii_l.reverse()
        }

        let img = image::open(path).unwrap(); 
        let mut img_buff: image::ImageBuffer<image::Rgba<u8>, Vec<u8>> = image::imageops::resize(&img, self.width, self.height, image::imageops::FilterType::Lanczos3);

        if img.width() >= self.width || img.height() >= self.height {
            img_buff = image::imageops::resize(&img, self.width, self.height, image::imageops::FilterType::Nearest);
        }

        let mut count_x: u32 = 0;
        let mut img_content = String::new();
        let density: f32 = (ASCII_LIST_LENGTH - 1) as f32;
        for p in img_buff.pixels() {
            if count_x == img_buff.width() {
                img_content.push('\n');
                count_x = 0; 
            }

            let mut rgb_color: f32 = (density * get_rgb_byte_color(p[0], p[1], p[2])).round();

            if p[3] == 0 {
                rgb_color = 0.0;
            }

            let mut ascii_val = ascii_l[rgb_color as usize].to_string();
            if self.colorized { 
                ascii_val  = format!("{}{}\x1B[0m", get_rgb_col(p[0], p[1], p[2]), ascii_l[rgb_color as usize]);
            }
            img_content.push_str(&ascii_val.into_boxed_str());
            count_x += 1;
        }
        img_content
    }
}

fn get_rgb_col(pixel_r: u8, pixel_g: u8, pixel_b: u8) -> String {
    format!("\x1B[38;2;{};{};{}m", pixel_r, pixel_g, pixel_b)
}

fn get_rgb_byte_color(pixel_r: u8, pixel_g: u8, pixel_b: u8) -> f32 {
    let pixel_r_f: f32 = (pixel_r / 3) as f32;
    let pixel_g_f: f32 = (pixel_g / 3) as f32;
    let pixel_b_f: f32 = (pixel_b / 3) as f32;
    let rgb_color: f32 = pixel_r_f + pixel_g_f + pixel_b_f;
    rgb_color / RGB_8_BYTE_VAL_FLOAT
}
