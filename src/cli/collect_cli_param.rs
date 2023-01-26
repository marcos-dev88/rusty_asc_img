use crate::cli::CliParams;
use std::collections::hash_map::Entry::{Occupied, Vacant};
use std::{collections::HashMap, fmt::Error};

const EMPTY_PARAM_VALUE: i32 = -1;

const PATH_FLAG: &str = "-p";
const WIDTH_FLAG: &str = "-w";
const HEIGHT_FLAG: &str = "-h";
const COLORIZED_FLAG: &str = "-c";
const REVERSED_FLAG: &str = "-rev";
const USE_BLOCK_ASCII_FLAG: &str = "-b";

pub fn get_params(params: Vec<String>) -> Result<CliParams, Error> {
    let mut cli_p: CliParams = CliParams::new();

    let map_index = match get_param_index(&params) {
        Ok(map_index) => map_index,
        Err(e) => {
            panic!("error to get params index: {}", e)
        }
    };

    if map_index.get(PATH_FLAG).is_some() && map_index[PATH_FLAG] != EMPTY_PARAM_VALUE {
        *cli_p.set_path() = match params[(map_index[PATH_FLAG] + 1) as usize].parse() {
            Ok(val) => val,
            Err(e) => {
                panic!("error on get path value: {}", e)
            }
        };
    }

    if map_index.get(WIDTH_FLAG).is_some() && map_index[WIDTH_FLAG] != EMPTY_PARAM_VALUE {
        *cli_p.set_width() = match params[(map_index[WIDTH_FLAG] + 1) as usize].parse() {
            Ok(val) => val,
            Err(e) => {
                panic!("error on get path value: {}", e)
            }
        };
    }

    if map_index.get(HEIGHT_FLAG).is_some() && map_index[HEIGHT_FLAG] != EMPTY_PARAM_VALUE {
        *cli_p.set_heigth() = match params[(map_index[HEIGHT_FLAG] + 1) as usize].parse() {
            Ok(val) => val,
            Err(e) => {
                panic!("error on get path value: {}", e)
            }
        };
    }

    if map_index.get(REVERSED_FLAG).is_some() && map_index[REVERSED_FLAG] != EMPTY_PARAM_VALUE {
        *cli_p.set_reversed_ascii() = true;
    }

    if map_index.get(COLORIZED_FLAG).is_some() && map_index[COLORIZED_FLAG] != EMPTY_PARAM_VALUE {
        *cli_p.set_colorized() = true;
    }

    if map_index.get(USE_BLOCK_ASCII_FLAG).is_some()
        && map_index[USE_BLOCK_ASCII_FLAG] != EMPTY_PARAM_VALUE
    {
        *cli_p.set_use_block_ascii() = true;
    }

    Ok(cli_p)
}

fn get_param_index(params: &[String]) -> Result<HashMap<&str, i32>, Error> {
    let mut map_index: HashMap<&str, i32> = Default::default();

    for (i, param) in params.iter().enumerate() {
        if param == PATH_FLAG
            || param == WIDTH_FLAG
            || param == HEIGHT_FLAG
            || param == REVERSED_FLAG
            || param == COLORIZED_FLAG
            || param == USE_BLOCK_ASCII_FLAG
        {
            let param_value = match map_index.entry(param) {
                Vacant(entry) => entry.insert(EMPTY_PARAM_VALUE),
                Occupied(entry) => entry.into_mut(),
            };

            *param_value = i as i32;
        }
    }

    Ok(map_index)
}
