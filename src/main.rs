use std::path::Path;
use std::{env, fs, io};
use std::fs::OpenOptions;
use std::io::Error;

fn create_file_if_not_exists(filename: &str) -> Result<(), Error> {
    match OpenOptions::new()
        .write(true)       
        .create_new(true)  
        .open(filename) 
    {
        Ok(_) => {
            println!("created new file: {}", filename);
            Ok(())
        },
        Err(e) => {
            if e.kind() == std::io::ErrorKind::AlreadyExists {
                println!("File already exists: {}", filename);
                Ok(()) 
            } else {
                eprintln!("Error creating file: {}", e);
                Err(e)
            }
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        eprintln!("Usage: cargo run --bin aoc -- new <year>-<day>");
        eprintln!("Example exec: ./aoc new 2025-2");
        return;
    }

    let command = &*args[1];

    let (year, day) = &args[2].split_once("-").expect("Could not split year/date from argument");

    if year.len() != 4 || day.len() > 2 || !day.chars().all(|c| c.is_ascii_digit()) || !year.chars().all(|c| c.is_ascii_digit()) {
        eprintln!("Arguments provided are not valid");
        return;
    }

    match command {
        "new" => {
            println!("Creating a new day: year={} day={}", year, day);

            let template_file = Path::new("src/bin/template.rs");

            let files : Vec<String> = vec![
                    format!("src/bin/{year}-{day}.rs"),
                    format!("data/{year}-{day}.txt"),
                    format!("puzzles/{year}-{day}.md")
                    ];

            for file in files {
                println!("Creating file: {}", file);
                match create_file_if_not_exists(&file) {
                    Ok(_) => println!("OK"),
                    Err(err) => eprintln!("{} failed to create with error: {}", file, err)
                };
            }

            println!("Are you sure you want to overwrite the file with template? [yes/no]");
            let mut overwrite_confirmation = String::new();

            io::stdin() 
                .read_line(&mut overwrite_confirmation) 
                .expect("Failed to read line");

            overwrite_confirmation = overwrite_confirmation.trim().to_lowercase();

            if overwrite_confirmation == "yes" || overwrite_confirmation == "y" {
                println!("\nOverwriting existing file with template file");
                match fs::copy(template_file, Path::new(&format!("src/bin/{year}-{day}.rs"))) {
                Ok(result) => println!("Copied {result} bytes"),
                Err(err) => eprintln!("Failed copying template to file with error: {err}")
            }
        } else {
            eprintln!("\nYou chose not to overwrite template");
        }

        },
        _ => eprintln!("Unknown command")
    }
}