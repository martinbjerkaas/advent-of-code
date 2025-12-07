use std::{collections::{HashMap, HashSet}, fs, path::Path};
use aoc::*;

const INPUT_FILE_PATH: &'static str = "data/2025-7.txt";

fn first_part(inputstring: &str) -> u32{
    let mut grid = Grid::new_from_string(inputstring, false);
    let mut beams: HashSet<Point> = HashSet::new();
    let mut splits = 0;

    let start = *grid.points.iter().find(|(_, character)| **character == 'S').expect("Could not find start pos").0;
    println!("Startpos: {:?}", start);

    beams.insert(start);
    
    for y in 0..grid.height {
        let mut new_beams : HashSet<Point> = HashSet::new();

        for beam in &beams {
            if beam.y == y {
                let next_down = grid.points.get(&beam.get_point_in_direction(Direction::Down));
                if next_down == Some(&'^') {
                    new_beams.insert(beam.get_point_in_direction(Direction::DiagonalDownLeft));
                    new_beams.insert(beam.get_point_in_direction(Direction::DiagonalDownRight));
                    println!("Splitted at: {} to {} {}", &beam, beam.get_point_in_direction(Direction::DiagonalDownLeft), beam.get_point_in_direction(Direction::DiagonalDownRight));
                    splits+=1;
                } else {
                    new_beams.insert(beam.get_point_in_direction(Direction::Down));
                }
            }
            
        }
        beams.extend(new_beams);    
    }

    for beam in beams {
        if beam != start {
            grid.change_cell_data_no_return(&beam, '|');
        }
    }

    println!("Splits: {}", splits);

    //fs::write(Path::new("output/2025-7.txt"), grid.get_grid_as_table());
    return splits
}

// PART TWO NOT SOLVED
fn second_part(inputstring: &str) -> u32 {
    let mut grid = Grid::new_from_string(inputstring, false);
    let mut beams: HashSet<Point> = HashSet::new();
    let mut splits = 0;

    let start = *grid.points.iter().find(|(_, character)| **character == 'S').expect("Could not find start pos").0;
    println!("Startpos: {:?}", start);

    beams.insert(start);
    
    for y in 0..grid.height {
        let mut new_beams : HashSet<Point> = HashSet::new();

        for beam in &beams {
            if beam.y == y {
                let next_down = grid.points.get(&beam.get_point_in_direction(Direction::Down));
                if next_down == Some(&'^') {
                    new_beams.insert(beam.get_point_in_direction(Direction::DiagonalDownLeft));
                    new_beams.insert(beam.get_point_in_direction(Direction::DiagonalDownRight));
                    println!("Splitted at: {} to {} {}", &beam, beam.get_point_in_direction(Direction::DiagonalDownLeft), beam.get_point_in_direction(Direction::DiagonalDownRight));
                    splits+=1;
                } else {
                    new_beams.insert(beam.get_point_in_direction(Direction::Down));
                }
            }
            
        }
        beams.extend(new_beams);    
    }

    // tar utgangspunkt i at alle beams i bunnen er et unikt startpunkt.
    let bottom_beams: &Vec<&Point> = &beams.iter().filter(|beam| beam.y == grid.height-2).collect();

    // traversere oppover, sjekk direction og husk posisjoner jeg har vært innom for hvert start punkt.
    let traverse_memory : HashMap<Point, Vec<Point>> = HashMap::new();    

    for start_pos in bottom_beams {
        let mut beam_pos = *start_pos;

        while beam_pos.y > 0 {
            let diagonal_left_char = &grid.points.get(&beam_pos.get_point_in_direction(Direction::DiagonalUpLeft)).expect("Could not get next point") == &&'|';
            let up_char = &grid.points.get(&beam_pos.get_point_in_direction(Direction::Up)).expect("Could not get up") == &&'|';
            let diagonal_right_char = &grid.points.get(&beam_pos.get_point_in_direction(Direction::DiagonalUpRight)).expect("Could not get diagonal up right") == &&'|';
        
            if diagonal_left_char || up_char || diagonal_right_char {
                
            }
        }
    };


    println!("Splits: {}", splits);
    println!("Bottom beams: {:?}", bottom_beams);

    
    return splits
}
fn main(){
    let test_string = ".......S.......\n...............\n.......^.......\n...............\n......^.^......\n...............\n.....^.^.^.....\n...............\n....^.^...^....\n...............\n...^.^...^.^...\n...............\n..^...^.....^..\n...............\n.^.^.^.^.^...^.\n...............";

    println!("First part example: {:?}", first_part(test_string));
    println!("Second part example: {:?}", second_part(test_string));

    match fs::read_to_string(Path::new(INPUT_FILE_PATH)) {
        Ok(file_content) => {
            //println!("First part file input: {:?}", first_part(&file_content));
            //println!("Second part file input: {:?}", second_part(&file_content));
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