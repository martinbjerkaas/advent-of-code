// Refactored 30/11-25
use std::{cmp::{max,min}, fs, path::Path};

const DEBUG : bool = false;

fn first_part(inputstring: &str) -> i32 {
    let parsed_string_as_vec: Vec<(i32,i32)> = inputstring.split_terminator('\n').filter_map(|line| {
        let splitted_line : Vec<&str>= line.split_ascii_whitespace().collect();

        if splitted_line.iter().count() == 2 {
            Some((
                splitted_line[0]
                .parse::<i32>().expect("Could not parse str"), 
                splitted_line[1].parse::<i32>()
                .expect("Could not parse second item")))
        } else {
            None
        }
    })
    .collect();

    if DEBUG {println!("Parsed string: \n{:?}", parsed_string_as_vec)};

    let (mut leftlist, mut rightlist): (Vec<i32>,Vec<i32>) = parsed_string_as_vec.into_iter().unzip();

    leftlist.sort();
    rightlist.sort();

    if DEBUG {println!("Leftlist length:{:?} Rightlist length:{:?}", &leftlist.len(), &rightlist.len())};

    let result: i32 = leftlist
    .into_iter().zip(rightlist)
    .inspect(|x| if DEBUG {println!("Checking difference between: {:?}", x)})
    .map(|(left, right)| max(left, right) - min(left,right))
    .inspect(|x| if DEBUG {println!("The difference was: {x}")})
    .sum();
    
    return result;
}

fn second_part(inputstring: &str) -> i32 {
    let (leftlist, rightlist): (Vec<i32>,Vec<i32>) = inputstring.split_terminator('\n').filter_map(|line| {
        let splitted_line : Vec<&str>= line.split_ascii_whitespace().collect();

        if splitted_line.iter().count() == 2 {
            Some((
                splitted_line[0]
                .parse::<i32>().expect("Could not parse str"), 
                splitted_line[1].parse::<i32>()
                .expect("Could not parse second item")))
        } else {
            None
        }
    })
    .unzip();

    let mut result : i32 = 0;

    for num in leftlist {
        let occurences: Vec<i32> = rightlist
        .iter()
        .copied()
        .filter(|x| *x == num)
        .collect();
        
        result += num * occurences.len() as i32;
    }
    
    return result;
}

fn main(){
    let test_string = "3   4\n4   3\n2   5\n1   3\n3   9\n3   3";

    println!("The total difference for the test string is: {}", first_part(test_string));

    let file_content = fs::read_to_string(Path::new("data/2024-1.txt")).expect("Could not read file");
    println!("The total difference for part 1 is: {}", first_part(&file_content));

    let file_content = fs::read_to_string(Path::new("data/2024-1.txt")).expect("Could not read file");
    println!("The total simulatity score for part 2 is: {}", second_part(&file_content));

}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn can_load_first_file() {
        let file_content = fs::read_to_string(Path::new("data/2024-1.txt"));

        assert!(file_content.is_ok());
    }

    #[test]
    fn first_part_is_correct() {
        let file_content = fs::read_to_string(Path::new("data/2024-1.txt")).expect("Could not load file");
        assert_eq!(first_part(&file_content), 765748);
    }

    #[test]
    fn second_part_is_correct() {
        let file_content = fs::read_to_string(Path::new("data/2024-1.txt")).expect("Could not load file");
        assert_eq!(second_part(&file_content), 27732508);
}

}