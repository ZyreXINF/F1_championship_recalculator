use std::io;
use std::process::exit;

pub fn init_cli(){
    println!("\nWelcome to F1 Championship Calculator");
    open_main_menu();
}

fn open_main_menu(){
    println!(
        "\nPlease select an option:\n\
        [1] - Start recalculation\n\
        [2] - Configure\n\
        [3] - Quit\n\
    ");
    match process_choice(3) {
        1 => {
            //TODO Invoke API, Start Calculations etc.
        }
        2 => open_configuration_menu(),
        3 => exit(0),
        _ => println!("Invalid data")
    }
}

fn open_configuration_menu() {
    println!(
        "\nPlease select an option:\n\
        [1] - Select a championship year\n\
        [2] - Select an existing point system\n\
        [3] - Import point system\n\
        [4] - View Settings\n\
        [5] - Back\n\
    ");
    match process_choice(5) {
        1 => {
            println!("Please type a year between 1950 and 2025:");
            input_championship_year();
            println!("\nChampionship year configuration saved successfully");
            //TODO Save data
            open_configuration_menu();
        },
        2 => {
            open_point_system_config();
        },
        3 => println!("Importing point system..."),
        4 => {
            println!("Current Settings: ???" );
            open_configuration_menu();
        },
        5 => open_main_menu(),
        _ => println!("Invalid data")
    }
}

fn open_point_system_config(){
    println!("\
            Please select an existing point system:\n\
            [1] - Modern system\n\
            [2] - Modern system (+Fastest Lap Point)\n\
            [3] - 2003-2009\n\
            [4] - 1991–2002
    ");
    //TODO Check GP point-system input and verify sprint point-systems
}

fn input_championship_year() -> u16{
    loop{
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read line");
        match choice.trim().parse::<u16>() {
            Ok(num) if (1950..=2025).contains(&num) => return num,
            _ => println!("Please enter a valid year between 1950 and 2025")
        };
    }
}
fn process_choice(max_value: u8) -> u8{
    loop{
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read line");
        match choice.trim().parse::<u8>() {
            Ok(num) if (1..=max_value).contains(&num) => return num,
            _ => println!("Please enter a valid number!")
        };
    }
}