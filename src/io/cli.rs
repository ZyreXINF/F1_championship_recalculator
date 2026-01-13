use std::process::exit;
use crate::model::app_state::AppState;
use crate::io::cli_input;
use crate::io::config_parser;
use crate::api::data_parser;
use crate::model::driver::ChampionshipPosition;
use crate::service::calculator::recalculate_championship;

#[tokio::main]
pub async fn init_cli(state: &mut AppState) {
    println!("\nWelcome to F1 Championship Calculator");
    open_main_menu(state).await;
}

async fn open_main_menu(state: &mut AppState){
    println!(
        "\nPlease select an option:\n\
        [1] - Start recalculation\n\
        [2] - Configure\n\
        [3] - Quit\n\
    ");
    match cli_input::process_choice(3) {
        1 => {
            println!("\nFetching results for {} championship...", state.settings.championship_year);

            let driver_results = if let Some(results) = state.race_results.saved.get(&state.settings.championship_year) { results.clone() }
                else{
                    let results = data_parser::request_drivers_results(&state.settings.championship_year).await.unwrap();
                    state.race_results.saved.insert(state.settings.championship_year, results.clone());
                    results
                };

                println!("\nRecalculating {} championship...", state.settings.championship_year);
            let standings = recalculate_championship(&state.settings.point_system, driver_results);
            Box::pin(view_standings(state, standings)).await;

            Box::pin(open_main_menu(state)).await;
        }
        2 => Box::pin(open_configuration_menu(state)).await,
        3 => exit(0),
        _ => println!("Invalid data")
    }
}

async fn view_standings(state: &AppState, standings: Vec<ChampionshipPosition>) {
    println!("\nChampionship Standings for {} year", state.settings.championship_year);
    println!("{:<4} {:<25} {:>6} {:>3} {:>4} {:>4} {:>5} {:>4} {:>5}",
        "Pos", "Driver", "Pts", "W", "SW", "FL", "FL(S)", "Pol", "Pol(S)");
    println!("{}", "-".repeat(70));

    for (i, driver) in standings.iter().enumerate() {
        println!(
            "{:<4} {:<25} {:>6} {:>3} {:>4} {:>4} {:>5} {:>4} {:>5}",
            i + 1,
            driver.full_name,
            driver.points_scored,
            driver.wins,
            driver.sprint_wins,
            driver.fastest_laps,
            driver.sprint_fastest_laps,
            driver.poles,
            driver.sprint_poles
        );
    }
}

async fn open_configuration_menu(state: &mut AppState) {
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
            Box::pin(open_configuration_menu(state)).await;
        },
        2 => {
            Box::pin(open_point_system_config(state)).await;
        },
        3 => {
            let file_path = cli_input::input_file_directory();
            state.settings.point_system = config_parser::parse_config(&file_path);
            println!("\nPoint system configuration saved successfully");
            Box::pin(open_configuration_menu(state)).await;
        },
        4 => {
            state.settings.view_settings();
            Box::pin(open_configuration_menu(state)).await;
        },
        5 => open_main_menu(state).await,
        _ => println!("Invalid data")
    }
}

async fn open_point_system_config(state: &mut AppState){
    println!("\
            Please select an existing point system:\n\
            [1] - Modern system\n\
            [2] - Modern system (+ Fastest lap point)\n\
            [3] - 2003-2009\n\
            [4] - 1991–2002
    ");
    match cli_input::process_choice(4) {
        1 => {
            state.settings.point_system = config_parser::parse_embedded_config("modern");
            println!("\nModern system configuration saved successfully");
        },
        2 => {
            state.settings.point_system = config_parser::parse_embedded_config("modern");
            state.settings.point_system.fastest_lap_point = 1;
            println!("\nModern system configuration (+ Fastest lap point) saved successfully");
        },
        3 => {
            state.settings.point_system = config_parser::parse_embedded_config("classic");
            println!("\nClassic system configuration saved successfully");
        },
        4 => {
            state.settings.point_system = config_parser::parse_embedded_config("legacy");
            println!("\nLegacy system configuration saved successfully");
        },
        _ => println!("Invalid data")
    }
    Box::pin(open_configuration_menu(state)).await;
}