// Refactored 30/11-25
use std::{fs, path::Path};

use aoc::*;

fn first_part(inputstring: &str) -> i32 {
    let grid = Grid::new_from_string(inputstring, false);
    let word = "XMAS";
    let directions_to_check = [
        Direction::Right, 
        Direction::Left, 
        Direction::Up,
        Direction::Down,
        Direction::DiagonalDownLeft,
        Direction::DiagonalDownRight,
        Direction::DiagonalUpRight,
        Direction::DiagonalUpLeft
        ];
    let mut appearances = 0;

    // Getting all starting positions
    let all_x_positions: Vec<Point> = grid.points
    .iter()
    .filter_map(|(point, data)| {
        if *data == 'X' {
            Some(*point)
        } else{
            None
        }
    })
    .collect();

    for x_pos in all_x_positions {
        for direction in &directions_to_check {
            let word_to_check = grid.get_string_in_direction(x_pos, word.len() as i32, *direction);
            match word_to_check {
                Ok(matched_word) => if word == matched_word {
                    appearances += 1;
                },
                Err(_e) => {} 
            }
        }
    }
    println!("Total appearances: {}", appearances);
    return appearances;

}

fn second_part(inputstring: &str) -> i32 {
    let grid = Grid::new_from_string(inputstring, false);
    let word = "MAS";
    let directions_to_check = [
        Direction::DiagonalDownLeft,
        Direction::DiagonalDownRight,
        Direction::DiagonalUpRight,
        Direction::DiagonalUpLeft
        ];
    let mut appearances = 0;

    // Getting all starting positions
    let all_start_positions: Vec<Point> = grid.points
    .iter()
    .filter_map(|(point, data)| {
        if *data == 'A' {
            Some(*point)
        } else{
            None
        }
    })
    .collect();

    // Offsets the start position. If it gets to matches, its considered a cross.
    for start_pos in all_start_positions {
        let mut matches: Vec<String> = Vec::new();
        for direction in &directions_to_check {
            let temp_word: Option<String> = match direction {
                Direction::DiagonalDownRight => {
                    grid.get_string_in_direction(start_pos + Point{x: -1 ,y: -1}, word.len() as i32, *direction).ok()
                },
                Direction::DiagonalDownLeft => {
                    grid.get_string_in_direction(start_pos + Point{x: 1 ,y: -1}, word.len() as i32, *direction).ok()
                },
                Direction::DiagonalUpLeft => {
                    grid.get_string_in_direction(start_pos + Point{x: 1 ,y: 1}, word.len() as i32, *direction).ok()
                },
                Direction::DiagonalUpRight => {
                    grid.get_string_in_direction(start_pos + Point{x: -1 ,y: 1}, word.len() as i32, *direction).ok()
                },
                _ => None,
            };

            if let Some(unwrapped_word) = temp_word {
                if unwrapped_word == word{
                    matches.push(unwrapped_word);
                }
            }
           
        }

        if matches.len() == 2 {
            appearances +=1
        }
    }

    println!("Total appearances: {}", appearances);
    return appearances;
}
fn main(){
    let test_string = "MMMSXXMASM\nMSAMXMSMSA\nAMXSXMAAMM\nMSAMASMSMX\nXMASAMXAMM\nXXAMMXXAMA\nSMSMSASXSS\nSAXAMASAAA\nMAMMMXMMMM\nMXMXAXMASX";
    first_part(test_string);
    second_part(test_string);

    let file_content = fs::read_to_string(Path::new("data/2024-4.txt")).expect("Could not load the file.");
    first_part(&file_content);
    second_part(&file_content);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_part_is_correct() {
        let file_content = fs::read_to_string(Path::new("data/2024-4.txt")).expect("Could not load file");
        assert_eq!(first_part(&file_content), 2560);
    }

    #[test]
    fn second_part_is_correct() {
        let file_content = fs::read_to_string(Path::new("data/2024-4.txt")).expect("Could not load file");
        assert_eq!(second_part(&file_content), 1910);
}

}