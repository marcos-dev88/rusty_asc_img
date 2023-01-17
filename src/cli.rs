 pub mod collect_cli_param;

 #[derive(Debug)]
 pub struct CliParams { 
    path: String,
    width: u32,
    heigth: u32,
}

impl CliParams {
    pub fn path(&self) -> &String {
        &self.path
    }

    pub fn width(&self) -> &u32{ 
        &self.width
    }

     pub fn heigth(&self) -> &u32{
        &self.heigth
    }
}

impl CliParams{ 
     fn set_path(&mut self) -> &mut String {
        &mut self.path
    }

     fn set_width(&mut self) -> &mut u32{ 
        &mut self.width
    }

     fn set_heigth(&mut self) -> &mut u32{
        &mut self.heigth
    }

}

