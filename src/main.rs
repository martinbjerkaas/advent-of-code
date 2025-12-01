use std::env;
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
        eprintln!("Usage: cargo run --bin aoc -- <year> <day>");
        return;
    }

    let year = &args[1];
    let day = &args[2];

    if year.len() != 4 || day.len() > 2 || !day.chars().all(|c| c.is_ascii_digit()) || !year.chars().all(|c| c.is_ascii_digit()) {
        eprintln!("Arguments provided are not valid");
        return;
    }

    println!("Creating a new day: year={} day={}", year, day);

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

    
}