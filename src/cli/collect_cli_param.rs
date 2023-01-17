use crate::cli::CliParams;

const DEFAULT_WIDTH: u32 = 75;
const DEFAULT_HEIGTH: u32 = 35;

const EMPTY_PARAM_VALUE: i32 = -1;

pub fn get_params(params: Vec<String>) -> CliParams { 
    let mut cli_p: CliParams = CliParams{path: String::new(), width: DEFAULT_WIDTH, heigth: DEFAULT_HEIGTH};

    let (path_index, w_index, h_index) = get_param_index(&params);

    if path_index != EMPTY_PARAM_VALUE { 
        *cli_p.set_path() = params[(path_index+1) as usize].parse().unwrap();
    }

    if w_index != EMPTY_PARAM_VALUE { 
        *cli_p.set_width() = params[(w_index+1) as usize].parse().unwrap();

    }

    if h_index != EMPTY_PARAM_VALUE { 
        *cli_p.set_heigth() = params[(h_index+1) as usize].parse().unwrap();
    }


    cli_p
}

fn get_param_index(params: &[String]) -> (i32, i32, i32){
    let mut path_index: i32 = EMPTY_PARAM_VALUE;
    let mut width_index: i32 = EMPTY_PARAM_VALUE;
    let mut heigth_index: i32 = EMPTY_PARAM_VALUE;

    for (i, param) in params.iter().enumerate(){

        if param == "-p"{
            path_index = i as i32
        }

        if param == "-w"{
            width_index = i as i32
        }

        if param == "-h"{
            heigth_index = i as i32
        }
    }

    (path_index, width_index, heigth_index)
}
