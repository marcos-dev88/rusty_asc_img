use crate::file::*;
use std::fs::OpenOptions;
use std::path::Path;
use std::{fs::File as RFile, io::Write};

impl File {
    pub fn save_to_path(&self) -> Result<bool> {
        if self.path().is_empty() {
            return Ok(false);
        }

        match check_file_name_by_path(self.path.to_string()) {
            Ok(_) => (),
            Err(e) => {
                return Err(new_err(e.status, e.message));
            }
        }

        let input_path = Path::new(self.path());

        if !input_path.exists() {
            match RFile::create(input_path) {
                Ok(mut f) => match f.write_all(&self.content) {
                    Ok(_) => (),
                    Err(e) => {
                        return Err(new_err(500, e.to_string()));
                    }
                },
                Err(e) => {
                    return Err(new_err(500, e.to_string()));
                }
            }
            return Ok(true);
        }

        let file_open = OpenOptions::new()
            .read(true)
            .write(true)
            .truncate(true)
            .open(input_path);

        match file_open {
            Ok(mut f) => match f.write_all(&self.content) {
                Ok(_) => (),
                Err(e) => {
                    return Err(new_err(500, e.to_string()));
                }
            },
            Err(e) => {
                return Err(new_err(500, e.to_string()));
            }
        }

        Ok(true)
    }
}

fn check_file_name_by_path(path: String) -> Result<bool> {
    let splt_path: Vec<_> = path.split('/').collect();

    for (i, spl) in splt_path.iter().enumerate() {
        if i == 0 {
            // It's continuing here because of main directory in linux like:
            // /home/your-user/your-file
            continue;
        }

        if spl.is_empty() {
            return Err(new_err(500, "not valid file name".to_string()));
        }
    }

    Ok(true)
}

#[cfg(test)]
mod test {
    use super::check_file_name_by_path;

    #[test]
    fn correct_file_name() {
        let path = String::from("some/path/file.rs");
        let result = check_file_name_by_path(path);
        assert!(result.is_ok())
    }

    #[test]
    fn incorrect_file_name() {
        let path = String::from("some/path/file/");
        let result = check_file_name_by_path(path);
        assert!(result.is_err())
    }
}
