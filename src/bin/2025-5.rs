use std::{cmp::{Ordering, min, max}, collections::{BTreeSet, HashSet, btree_set::Range}, fs, ops::{RangeFrom, RangeInclusive}, path::Path};
//use aoc::*;

const INPUT_FILE_PATH: &'static str = "data/2025-5.txt";

fn first_part(inputstring: &str) -> u64 {
    // Split input into two lists -> for all ids, check if contains in each range -> if in range include in iterator and count.
    let (ranges, ids) = inputstring.split_once("\n\n").expect("Could not split by empty newline");
    let converted_ranges : Vec<_> = ranges.lines().filter_map(|range_as_str| {
        let (start_range, end_range) = range_as_str.split_once("-").expect("Could not parse str to range");
        Some(start_range.parse::<u64>().expect("Could not parse start range")..=end_range.parse::<u64>().expect("Could not parse end range"))
    }
    ).collect();

    let fresh_ids  = ids.lines().filter_map(|id| {
        let parsed_id = id.parse::<u64>().expect("Could not parse id");
        let mut result = None;

        for range in &converted_ranges {
            if range.contains(&parsed_id) {
                result = Some(parsed_id)
            }
        }

        result
    }).count();

    return fresh_ids as u64;

}

fn second_part(inputstring: &str) -> u64 {  
    // sorter alle ranges -> merge alle ranges -> flat ut alle innenfor range, bør gi unike verdier, dersom ikke, hiv de inn i BTreeSet
    let (ranges, _ids) = inputstring.split_once("\n\n").expect("Could not split by empty newline");
    let mut all_ranges: Vec<RangeInclusive<u64>> = ranges.lines()
        .filter_map(|range| {
            let (start,end) = range.split_once("-").expect("Could not split range to tuple");
            Some((start.parse::<u64>().ok()?..=end.parse::<u64>().ok()?))
        }).collect();

    println!("Uncompressed ranges: {}", all_ranges.iter().count());
    
    all_ranges.sort_by_key(|range| *range.start());
    //println!("All ranges: {:?}",all_ranges);

    let mut compressed_ranges: Vec<RangeInclusive<u64>> = Vec::new();

    for range in all_ranges {
        if let Some(last) = compressed_ranges.last_mut() {
            if range.start() <= last.end() { // sjekker om det er noe overlap
                let new_end = (*last.end()).max(*range.end());
                *last = (*last.start())..=new_end;
                continue;
            }
        }

        compressed_ranges.push(range); // dersom ikke ikke 
    }

    println!("Compressed ranges: {:?}",compressed_ranges.iter().count());

    let fresh_ids = compressed_ranges.into_iter().flatten().count();   
    return fresh_ids as u64;

     
/*     let fresh_ids : HashSet<u64> = HashSet::from_iter(compressed_ranges.into_iter().flatten());    

    println!("{}", fresh_ids.iter().count()); */
    
    // 1448370698 to low

}
fn main(){
    let test_string = "3-5\n10-14\n16-20\n12-18\n\n1\n5\n8\n11\n17\n32";

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
        assert_eq!(first_part(&file_content), 611);
    }

    #[test]
    fn second_part_is_correct() {
        let file_content = fs::read_to_string(Path::new(INPUT_FILE_PATH)).expect("Could not load file");
        assert_eq!(second_part(&file_content), 345995423801866);
}

}