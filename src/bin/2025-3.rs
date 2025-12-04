use std::{fs, path::Path};

const INPUT_FILE_PATH : &'static str = "data/2025-3.txt";

fn first_part(inputstring: &str) -> i32 {
    let batteries : i32 = inputstring.lines()
            .enumerate()
            .filter_map(|(_index,line)| {
                let line_as_bytes = line.as_bytes();
                let first_max = line_as_bytes[0..line_as_bytes.len()-1].iter().max().expect("Could not get first max");
                let first_max_index = line_as_bytes.iter().position(|&b| b==*first_max).expect("Could not get index of first max");
                let next_max_slice = &line_as_bytes[first_max_index+1..line_as_bytes.len()];
                let next_max = next_max_slice.iter().max().expect("Could not get next max");

                println!("Line as bytes: {:?} - Index: {} - Max: {:?} - Partial byte slice: {:?} - Next max: {:?}",line_as_bytes,first_max_index,*first_max as char,next_max_slice,*next_max as char);

                let number: i32 = String::from_utf8(
                    [*first_max,*next_max]
                    .to_vec()).expect("Could not create string from char bytes")
                    .parse().expect("Could not parse number as i32");
                println!("{:?}",number);

                Some(number)
            }
            )
            .sum();

    return batteries;
}

// TODO:refaktorere til iter med .map
fn second_part(inputstring: &str) -> u64 {
    let batteries = inputstring.lines().map(|line| line.as_bytes());
    let sequence_length = 12;
    let mut total_joltage: u64 = 0;

    for batterybank in batteries {
        let mut highest_byte_sequence: Vec<u8> = Vec::with_capacity(sequence_length);
        let bank_length = batterybank.len();
        let mut current_index = 0;

        while highest_byte_sequence.len() < sequence_length {
            let remaining_to_pick = sequence_length - highest_byte_sequence.len();
            let last_valid_index = bank_length - remaining_to_pick;

            let mut best_index = current_index;
            for i in current_index..=last_valid_index {
                if batterybank[i] > batterybank[best_index] {
                    best_index = i;
                }
            }

            highest_byte_sequence.push(batterybank[best_index]);
            current_index = best_index + 1;
        }

        let num: u64 = String::from_utf8(highest_byte_sequence.clone())
            .expect("Could not parse byte array to string")
            .parse()
            .expect("Could not parse result as u64");

        total_joltage += num;

        //println!("Highest byte seq: {:?} As string: {}",highest_byte_sequence,String::from_utf8(highest_byte_sequence.clone()).expect("Could not parse byte array to string"));
    }

    //println!("Total: {}", total_joltage);
    total_joltage
}

fn main(){
    let test_string = "987654321111111\n811111111111119\n234234234234278\n818181911112111";

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

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn first_part_is_correct() {
        let file_content = fs::read_to_string(Path::new(INPUT_FILE_PATH)).expect("Could not load file");
        assert_eq!(first_part(&file_content), 17332);
    }

    #[test]
    fn second_part_is_correct() {
        let file_content = fs::read_to_string(Path::new(INPUT_FILE_PATH)).expect("Could not load file");
        assert_eq!(second_part(&file_content), 172516781546707);
}

}