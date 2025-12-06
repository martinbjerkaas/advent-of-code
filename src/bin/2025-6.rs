use std::{collections::{BTreeMap, HashMap}, fs, path::Path};
//use aoc::*;

const INPUT_FILE_PATH: &'static str = "data/2025-6.txt";

#[derive(Debug)]
struct MathProblem {
    numbers: Vec<u64>,
    operator: char
}

impl MathProblem {
    fn solve(&self) -> Result<u64, String>{
        match self.operator {
            '*' => {
                let mut last_num: u64 = 1;
                for num in self.numbers.iter() {
                    last_num *= num
                }
                return Ok(last_num)
            },
            '+' => {
                let mut last_num: u64 = 0;
                for num in self.numbers.iter() {
                    last_num += num
                }
                return Ok(last_num)
            },
            _ => return Err("Invalid operator".to_string())
        }
    }
}


fn first_part(inputstring: &str) -> u64{
    let parsed_lines  = inputstring.lines().filter_map( |line| {
        Some(line.split_ascii_whitespace().collect::<Vec<&str>>())
    });

    let mut columns: HashMap<u16, Vec<&str>> = HashMap::new();

    for row in parsed_lines{
        for (column_index ,number_as_str) in row.iter().enumerate() {
            columns.entry(column_index as u16).or_insert_with(Vec::new).push(&number_as_str);
        }
    }

    let result = columns.iter_mut().filter_map(|column| {
        let operator = column.1.pop().expect("Could not pop operator").chars().next().expect("Failed to convert to char");

        Some(
            MathProblem {
                numbers: column.1.iter().map(|num_as_str|num_as_str.parse::<u64>().expect("Could not parse str as u32")).collect(),
                operator: operator
            }.solve().ok()?
        )
    }).sum();

    return result;
}

fn second_part(inputstring: &str) -> u64 {
    // delen under lager en liste over indexer og lengder siden input filen har variable kolonne bredder.
    // forsøkte å iterere igjennom linjene for å anslå kolonne bredde, men det var problematisk å anslå bredde basert på mellomrom.
    let inputlines_without_ws  = inputstring.lines().filter_map( |line| {
        println!("Line: {}", line);
        Some(line.split_ascii_whitespace().collect::<Vec<&str>>())
    });

    let mut columns: HashMap<u16, Vec<&str>> = HashMap::new();
    let mut column_index_lengths : BTreeMap<u16, u8> = BTreeMap::new();

    for row in inputlines_without_ws{
        for (column_index ,number_as_str) in row.iter().enumerate() {
            columns.entry(column_index as u16).or_insert_with(Vec::new).push(&number_as_str);
        }
    }

    // lager en hashmap som holder på bredden for hver kolonne
    for (index, column_numbers) in columns {
        let new_max = column_numbers.iter().map(|number_as_str| number_as_str.len()).max().expect("Could not get max") as u8;

        column_index_lengths.entry(index).and_modify(|existing_max| {
            
            if new_max > *existing_max {
                *existing_max = new_max
            }
        }).or_insert(new_max);
    }

    // delen under henter ut hver tekstdel inkl. whitespace basert på kolonnebredden.

    let mut columns_with_whitespaces: BTreeMap<u16, Vec<&str>> = BTreeMap::new();

    for row in inputstring.lines() {
        let mut next_start = 0;
        for (index, length) in column_index_lengths.clone() {
            let current_slice = next_start..next_start+length as usize;
            let current_number_as_str = &row[current_slice.clone()];
            columns_with_whitespaces.entry(index).or_insert_with(Vec::new).push(&current_number_as_str);
            next_start = next_start+1 + length as usize;
            
        }
    }

    println!("{:?}", &columns_with_whitespaces);

    let mut parsed_list:Vec<MathProblem> = Vec::new();

    for (index,  numbers_as_str) in columns_with_whitespaces.iter_mut() {
        let length = column_index_lengths.get(&index).expect("Could not get length at index");
        let operator = numbers_as_str.pop().expect("Could not extract operator").chars().next().expect("Couldnt convert &str to char");
        let mut numbers : Vec<u64> = Vec::new();

        for index in (0..*length as usize).rev() {
            let mut modified_numbers: String = String::new();

            for num in numbers_as_str.iter() {
                modified_numbers.push_str(&num[index..index+1]);
            }
            numbers.push(modified_numbers.trim().parse::<u64>().expect("msg"));
        }
        println!("Number: {:?} Operator: {:?}", numbers, operator);
        parsed_list.push(MathProblem { numbers, operator });
    }

    return parsed_list.iter().map(|problem| problem.solve().expect("Couldnt solve math problem")).sum();
}
fn main(){
    let test_string = "123 328  51 64 \n 45 64  387 23 \n  6 98  215 314\n*   +   *   +  ";

    println!("#### PART ONE ####");
    println!("First part example: {:?}", first_part(test_string));

    println!("\n#### PART TWO ####");
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
        assert_eq!(first_part(&file_content), 5524274308182);
    }

    #[test]
    fn second_part_is_correct() {
        let file_content = fs::read_to_string(Path::new(INPUT_FILE_PATH)).expect("Could not load file");
        assert_eq!(second_part(&file_content), 8843673199391);
}

}