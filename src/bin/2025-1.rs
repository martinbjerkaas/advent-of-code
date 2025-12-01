use std::{char, fs, path::Path};

fn first_part(inputstring: &str) -> i32 {
    let instructions: Vec<(char,i32)> = inputstring
    .lines()
    .map(|line| line.split_at(1))
    .filter_map(|(direction, turns)| {
        // Siden linjen splittes ved index 1, så vil alltid direction.len() være 1.
        Some((direction.chars().last().expect("Could not convert the direction to char"), turns.parse::<i32>().expect("Could not parse turns")))
    })
    .collect();

    println!("{:?}", instructions);
    let mut current_dial_pos = 50;
    let mut zero_counter = 0;

    for (direction, turns) in instructions {
        let signed_turns = match direction {
            'L' =>  -turns, //{ current_dial_pos = ((current_dial_pos - ( turns % 100 ) % 100) + 100) % 100},
            'R' => turns, //{ current_dial_pos = (current_dial_pos + turns) % 100}
            _ => 0
        };

        current_dial_pos = (current_dial_pos + signed_turns) % 100;

        if current_dial_pos == 0 {
            zero_counter +=1
        }
        println!("The dial is rotated {}{} to point at {}", direction, turns, current_dial_pos);
    }
    return zero_counter;
}

fn second_part(inputstring: &str) -> i32 {
    let instructions: Vec<(char,i32)> = inputstring
    .lines()
    .map(|line| line.split_at(1))
    .filter_map(|(direction, turns)| {
        // Siden linjen splittes ved index 1, så vil alltid direction.len() være 1.
        Some((direction.chars().last().expect("Could not convert the direction to char"), turns.parse::<i32>().expect("Could not parse turns")))
    })
    .collect();

    let mut zero_counter: i32 = 0;
    let mut current_dial_pos: i32 = 50;

    for (direction, turns) in instructions {
        let old_dial_pos = current_dial_pos;

        let signed_turns = match direction {
            'L' => -turns,
            'R' => turns,
            _ => 0,
        };

        current_dial_pos += signed_turns;

        match current_dial_pos {
            x if (x > 99) => zero_counter+= current_dial_pos / 100,
            x if (x <= 0) => {
                zero_counter+= (current_dial_pos / 100).abs() +1;
                if old_dial_pos == 0 {
                    zero_counter -= 1;
                }
            }
            _ => continue
        }

        current_dial_pos = ((current_dial_pos % 100) + 100) % 100 // positiv modulo. Har rust noe innebygd?
    }

    return zero_counter;
}

fn main(){
    let test_string = "L68\nL30\nR48\nL5\nR60\nL55\nL1\nL99\nR14\nL82";
    let file_content = fs::read_to_string(Path::new("data/2025-1.txt")).expect("Could not load file");
    println!("Example string zero counter, first part: {}",first_part(test_string));
    println!("File content zero counter, first part: {}",first_part(&file_content));
    println!("Example string zero counter, second part: {}",second_part(test_string));
    println!("File content zero counter, second part: {}",second_part(&file_content));

    // 6341 to low
    // 6623
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn first_part_is_correct() {
        let file_content = fs::read_to_string(Path::new("data/2025-1.txt")).expect("Could not load file");
        assert_eq!(first_part(&file_content), 1132);
    }

    #[test]
    fn second_part_is_correct() {
        let file_content = fs::read_to_string(Path::new("data/2025-1.txt")).expect("Could not load file");
        assert_eq!(second_part(&file_content), 6623);
}

}