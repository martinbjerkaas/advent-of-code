use std::{collections::BTreeMap, fs, path::Path};

#[derive(Debug, PartialEq, Eq)]
enum Instruction {
    Start,
    End,
    Do,
    Dont,
    Unknown,
    Init
}

impl Instruction {
    fn as_pattern(&self) -> &'static str {
        match self {
            Instruction::Start => "mul(",
            Instruction::End => ")",
            Instruction::Do => "do()",
            Instruction::Dont => "don't()",
            _ => "N/A"
        }
    }

    fn from_pattern(pattern: &str) -> Instruction {
        match pattern {
             "mul(" => Instruction::Start,
             ")" => Instruction::End,
            "do()" => Instruction::Do,
            "don't()" => Instruction::Dont,
            _ => Instruction::Unknown
             
        }
    }
}

fn calculate_multiplication_total(inputstring: &String, use_new_instructions: bool) -> i32{
    let start_pos_matches = inputstring.match_indices(Instruction::Start.as_pattern()).into_iter();
    let end_pos_matches = inputstring.match_indices(Instruction::End.as_pattern()).into_iter();
    let do_pos_matches = inputstring.match_indices(Instruction::Do.as_pattern()).into_iter();
    let dont_pos_matches = inputstring.match_indices(Instruction::Dont.as_pattern()).into_iter();

    let all_matches = start_pos_matches.chain(end_pos_matches).chain(do_pos_matches).chain(dont_pos_matches);

    let mut sorted_matches: BTreeMap<usize, &str> = BTreeMap::new();

    for (index, pos_match) in all_matches {
        sorted_matches.insert(index, pos_match);
    }

    println!("{:#?}", sorted_matches);

    let mut dont_instruction_active = false;
    let mut candidates: Vec<&str> = Vec::new();
    let mut last_instruction: Instruction = Instruction::Init;
    let mut last_index = 0;
    let mut total_count = 0;

    for (index, pattern) in sorted_matches {
        println!("[Debug] Index: {} Pattern: {} Current instruction: {:?} Last instruction: {:?} Last index: {}", index,pattern,Instruction::from_pattern(pattern), last_instruction,last_index);
        match Instruction::from_pattern(pattern) {
            Instruction::End => {
                if (last_instruction == Instruction::Start) && (!dont_instruction_active || !use_new_instructions) {
                    candidates.push(&inputstring[last_index..index+1]);
                } else {
                    println!("This is an end, but last was not a start")
                }
            },
            Instruction::Do => {
                dont_instruction_active = false;
                println!("Switching to do {}", dont_instruction_active);
            },
            Instruction::Dont => { 
                dont_instruction_active = true;
                println!("DONT INSTRUCTION ACTIVE = {}", dont_instruction_active);
            }
            _ => println!("Doing nothing")
        }
       last_instruction = Instruction::from_pattern(pattern);
       last_index = index;
    }

    for candidate in candidates {
        
        let splitted_candidate: Vec<&str> = candidate[Instruction::Start.as_pattern().len()..candidate.len()-1]
        .split(",")
        //.filter_map(|number_as_str| number_as_str.parse::<i32>().ok())
        .collect();


        if splitted_candidate.len() == 2 {
            let clean_candidate: Vec<i32> = candidate[Instruction::Start.as_pattern().len()..candidate.len()-1]
            .split(",")
            .filter_map(|number_as_str| number_as_str.parse::<i32>().ok())
            .collect();
            
            if clean_candidate.len() == 2 {
                print!("Candidate: {}  ->  ", candidate);
                println!("Splitted candidate: {:?} included: {}", splitted_candidate, if splitted_candidate.len()==2{true}else{false});
                total_count += clean_candidate[0] * clean_candidate[1];
        }
        }
    }    


    return total_count;
}

fn main(){

    let test_string = String::from("xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))");
    let part_two_test_string = String::from("xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))");

    println!("Checking the test string: {:?}",assert_eq!(calculate_multiplication_total(&test_string,false),161));
    println!("Checking the part two test string: {:?}",assert_eq!(calculate_multiplication_total(&part_two_test_string, true),48));

    let file_content = fs::read_to_string(Path::new("data/day3.txt")).expect("Could not load the file");

    println!("Checking part 1: {:?}",assert_eq!(calculate_multiplication_total(&file_content,false),160672468));
    println!("Checking part 2: {:?}",assert_eq!(calculate_multiplication_total(&file_content, true),84893551));

    // 98049378 too high
    // 80615193 too low
    // correct: 84893551
    
}


