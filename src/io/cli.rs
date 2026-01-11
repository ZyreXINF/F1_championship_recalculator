use std::process::exit;
use crate::model::app_state::AppState;
use crate::io::cli_input;
use crate::io::config_parser;

pub fn init_cli(state: &mut AppState) {
    println!("\nWelcome to F1 Championship Calculator");
    open_main_menu(state);
}

fn open_main_menu(state: &mut AppState){
    println!(
        "\nPlease select an option:\n\
        [1] - Start recalculation\n\
        [2] - Configure\n\
        [3] - Quit\n\
    ");
    match cli_input::process_choice(3) {
        1 => {
            //TODO Invoke API, Start Calculations etc.
        }
        2 => open_configuration_menu(state),
        3 => exit(0),
        _ => println!("Invalid data")
    }
}

fn open_configuration_menu(state: &mut AppState) {
    println!(
        "\nPlease select an option:\n\
        [1] - Select a championship year\n\
        [2] - Select an existing point system\n\
        [3] - Import point system\n\
        [4] - View Settings\n\
        [5] - Back\n\
    ");
    match cli_input::process_choice(5) {
        1 => {
            println!("Please type a year between 1950 and 2025:");
            state.settings.championship_year = cli_input::input_championship_year();
            println!("\nChampionship year configuration saved successfully");
            open_configuration_menu(state);
        },
        2 => {
            open_point_system_config(state);
        },
        3 => {
            let file_path = cli_input::input_file_directory();
            state.settings.point_system = config_parser::parse_config(&file_path);
            println!("\nPoint system configuration saved successfully");
            open_configuration_menu(state);
        },
        4 => {
            state.settings.view_settings();
            open_configuration_menu(state);
        },
        5 => open_main_menu(state),
        _ => println!("Invalid data")
    }
}

fn open_point_system_config(state: &mut AppState){
    println!("\
            Please select an existing point system:\n\
            [1] - Modern system\n\
            [2] - Modern system (+ Fastest lap point)\n\
            [3] - 2003-2009\n\
            [4] - 1991–2002
    ");
    match cli_input::process_choice(4) {
        1 => {
            state.settings.point_system = config_parser::parse_config("default_configs/modern.toml");
            println!("\nModern system configuration saved successfully");
        },
        2 => {
            state.settings.point_system = config_parser::parse_config("default_configs/modern.toml");
            state.settings.point_system.fastest_lap_point = 1;
            println!("\nModern system configuration (+ Fastest lap point) saved successfully");
        },
        3 => {
            state.settings.point_system = config_parser::parse_config("default_configs/f1_2003_2009.toml");
            println!("\n2003-2009 system configuration saved successfully");
        },
        4 => {
            state.settings.point_system = config_parser::parse_config("default_configs/f1_1991_2002.toml");
            println!("\n1991-2002 system configuration saved successfully");
        },
        _ => println!("Invalid data")
    }
    open_configuration_menu(state);
}