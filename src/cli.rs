pub mod collect_cli_param;

const DEFAULT_WIDTH: u32 = 75;
const DEFAULT_HEIGTH: u32 = 35;

#[derive(Debug)]
pub struct CliParams {
    path: String,
    save_file_path: String,
    width: u32,
    heigth: u32,
    reversed_ascii: bool,
    colorized: bool,
    use_block_ascii: bool,
}

impl CliParams {
    pub fn path(&self) -> &String {
        &self.path
    }

    pub fn save_file_path(&self) -> &String {
        &self.save_file_path
    }

    pub fn width(&self) -> &u32 {
        &self.width
    }

    pub fn heigth(&self) -> &u32 {
        &self.heigth
    }

    pub fn reversed_ascii(&self) -> &bool {
        &self.reversed_ascii
    }

    pub fn colorized(&self) -> &bool {
        &self.colorized
    }

    pub fn use_block_ascii(&self) -> &bool {
        &self.use_block_ascii
    }
}

impl CliParams {
    fn set_path(&mut self) -> &mut String {
        &mut self.path
    }

    fn set_save_file_path(&mut self) -> &mut String {
        &mut self.save_file_path
    }

    fn set_width(&mut self) -> &mut u32 {
        &mut self.width
    }

    fn set_heigth(&mut self) -> &mut u32 {
        &mut self.heigth
    }

    fn set_reversed_ascii(&mut self) -> &mut bool {
        &mut self.reversed_ascii
    }

    fn set_colorized(&mut self) -> &mut bool {
        &mut self.colorized
    }

    fn set_use_block_ascii(&mut self) -> &mut bool {
        &mut self.use_block_ascii
    }
}

impl CliParams {
    pub fn new() -> CliParams {
        CliParams {
            path: String::new(),
            save_file_path: String::new(),
            width: DEFAULT_WIDTH,
            heigth: DEFAULT_HEIGTH,
            reversed_ascii: false,
            colorized: false,
            use_block_ascii: false,
        }
    }
}
