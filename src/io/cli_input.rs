use std::io;
use std::path::Path;

pub fn input_championship_year() -> u16{
    loop{
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read line");
        match choice.trim().parse::<u16>() {
            Ok(num) if (1950..=2025).contains(&num) => return num,
            _ => println!("Please enter a valid year between 1950 and 2025")
        };
    }
}

pub fn input_file_directory() -> String{
    loop{
        println!("\nPlease enter a path to a .toml file: (e.g. C:/User/points.toml)");
        let mut input_dir = String::new();
        io::stdin().read_line(&mut input_dir).expect("Failed to read line");
        let input_dir = input_dir.trim();
        if input_dir.is_empty() {
            println!("Please enter a path");
            continue;
        }
        let directory = Path::new(input_dir);
        if !directory.exists() {
            println!("Such directory does not exist, please enter a valid directory");
            continue;
        }
        if let Some(ext) = directory.extension() {
            if ext != "toml" {
                println!("File must have a .toml extension, please try again");
            }else{
                return input_dir.to_string();
            }
        }else{
            println!("Please enter a path to .toml file");
        }
    }
}

pub fn process_choice(max_value: u8) -> u8{
    loop{
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read line");
        match choice.trim().parse::<u8>() {
            Ok(num) if (1..=max_value).contains(&num) => return num,
            _ => println!("Please enter a valid number!")
        };
    }
}