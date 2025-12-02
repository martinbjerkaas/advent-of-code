use std::{fs, path::Path};

fn first_part(inputstring: &str) -> i64 {
    // Splitt inputstring til range -> split hver range -> parse til i64 og lag en vec av nummere inne i range inkl.slutt -> pakk ut i en vec
    let all_numbers: Vec<i64> = inputstring
            .split_terminator(',')
            .flat_map(|line| {
                let (start_range, end_range) = line.split_once('-').expect("Could not parse the line");
                let start = start_range.parse::<i64>().expect("Could not parse start_range to i64");
                let end = end_range.parse::<i64>().expect("Could not parse start_range to i64");

                (start..=end).collect::<Vec<i64>>()
    })
    .collect();

    let sum_invalid_ids: i64  = all_numbers.iter()
    .filter(|num| (num.to_string().len() % 2 == 0) && num.to_string()[0..num.to_string().len() / 2].repeat(2) == num.to_string()).sum();
    
    return sum_invalid_ids;
}

fn second_part(inputstring: &str) -> i64 {
    // Splitt inputstring til range -> split hver range -> parse til i64 og lag en vec av nummere inne i range inkl.slutt -> pakk ut i en vec
    let all_numbers: Vec<i64> = inputstring
            .split_terminator(',')
            .flat_map(|line| {
                let (start_range, end_range) = line.split_once('-').expect("Could not parse the line");
                let start = start_range.parse::<i64>().expect("Could not parse start_range to i64");
                let end = end_range.parse::<i64>().expect("Could not parse start_range to i64");

                (start..=end).collect::<Vec<i64>>()
            })
            .collect();

    let sum_invalid_ids: i64 = all_numbers
            .iter()
            .filter_map(|num| {
                let num_as_str = num.to_string();

                // sjekker hvor mange like deler nummeret kan deles opp i 1231231213 = 9 -> 3 og 9
                for pattern_length in 2..=num_as_str.len() {
                    let split_index = num_as_str.len() / pattern_length;

                    if num_as_str.len() % pattern_length != 0 { // Om den ikke kan deles likt, hopp videre
                        continue;
                    }

                    let pattern_str = &num_as_str[0..split_index];
                    let repeated_pattern = pattern_str.repeat(pattern_length);

                    if repeated_pattern == num_as_str {
                        return Some(*num);
                    }
                }

                None
            })
            .sum();
    
    return sum_invalid_ids;
}

fn main() {
    let test_string = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

    println!("First part example: {}",first_part(test_string));
    println!("Second part example: {}",second_part(test_string));

    let file_content = fs::read_to_string(Path::new("data/2025-2.txt")).expect("Could not load file");
    println!("First part inputfile: {}", first_part(&file_content));
    println!("Second part inputfile: {}", second_part(&file_content));
}


#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn first_part_is_correct() {
        let file_content = fs::read_to_string(Path::new("data/2025-2.txt")).expect("Could not load file");
        assert_eq!(first_part(&file_content), 15873079081);
    }

    #[test]
    fn second_part_is_correct() {
        let file_content = fs::read_to_string(Path::new("data/2025-2.txt")).expect("Could not load file");
        assert_eq!(second_part(&file_content), 22617871034);
}

}