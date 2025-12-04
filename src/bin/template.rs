use std::{fs, path::Path};
//use aoc::*;

const INPUT_FILE_PATH: &'static str = "data/.txt";

fn first_part(inputstring: &str) -> String{
    inputstring.to_string()
}

fn second_part(inputstring: &str) -> String {
    inputstring.to_string()
}
fn main(){
    let test_string = "";

    println!("First part example: {:?}", first_part(test_string));
    println!("Second part example: {:?}", second_part(test_string));

    match fs::read_to_string(Path::new(INPUT_FILE_PATH)) {
        Ok(file_content) => {
            println!("First part file input: {:?}", first_part(&file_content));
            println!("Second part file input: {:?}", second_part(&file_content));
        },
        Err(err) => eprintln!("Could not find inputfile with error: {err}")
    }



}

/* #[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn first_part_is_correct() {
        let file_content = fs::read_to_string(Path::new(INPUT_FILE_PATH)).expect("Could not load file");
        assert_eq!(first_part(&file_content), 15873079081);
    }

    #[test]
    fn second_part_is_correct() {
        let file_content = fs::read_to_string(Path::new(INPUT_FILE_PATH)).expect("Could not load file");
        assert_eq!(second_part(&file_content), 22617871034);
}

} */