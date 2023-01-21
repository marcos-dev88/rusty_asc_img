use crate::cli::CliParams;
use std::collections::HashMap;

const DEFAULT_WIDTH: u32 = 75;
const DEFAULT_HEIGTH: u32 = 35;
const EMPTY_PARAM_VALUE: i32 = -1;

pub fn get_params(params: Vec<String>) -> CliParams { 
    let mut cli_p: CliParams = CliParams{path: String::new(), width: DEFAULT_WIDTH, heigth: DEFAULT_HEIGTH, reversed_ascii: false, colorized: false};
    let map_index = get_param_index(&params);

    if map_index["-p"] != EMPTY_PARAM_VALUE { 
        *cli_p.set_path() = params[(map_index["-p"]+1) as usize].parse().unwrap();
    }

    if map_index["-w"] != EMPTY_PARAM_VALUE { 
        *cli_p.set_width() = params[(map_index["-w"]+1) as usize].parse().unwrap();
    }

    if map_index["-h"] != EMPTY_PARAM_VALUE { 
        *cli_p.set_heigth() = params[(map_index["-h"]+1) as usize].parse().unwrap();
    }

    if map_index["-rev"] != EMPTY_PARAM_VALUE { 
        *cli_p.set_reversed_ascii() = true;
    }

    if map_index["-c"] != EMPTY_PARAM_VALUE { 
        *cli_p.set_colorized() = true;
    } 

    cli_p
}

fn get_param_index(params: &[String]) -> HashMap<&str, i32>{
    let mut map_index: HashMap<&str, i32> = Default::default();

    map_index.insert("-p", EMPTY_PARAM_VALUE);
    map_index.insert("-w", EMPTY_PARAM_VALUE);
    map_index.insert("-h", EMPTY_PARAM_VALUE);
    map_index.insert("-rev", EMPTY_PARAM_VALUE);
    map_index.insert("-c", EMPTY_PARAM_VALUE);
 
    for (i, param) in params.iter().enumerate(){

        if param == "-p"{
            *map_index.get_mut("-p").unwrap() = i as i32;
        }

        if param == "-w"{
            *map_index.get_mut("-w").unwrap() = i as i32;
        }

        if param == "-h"{
            *map_index.get_mut("-h").unwrap() = i as i32;
        }

        if param == "-rev"{
            *map_index.get_mut("-rev").unwrap() = i as i32;
        }

        if param == "-c"{
            *map_index.get_mut("-c").unwrap() = i as i32;
        }
    }

    map_index
}
