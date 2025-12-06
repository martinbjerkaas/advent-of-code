use std::{fs, path::Path};

use aoc::*;

const INPUT_FILE_PATH: &'static str = "data/2025-4.txt";

fn first_part(inputstring: &str) -> u64 {
    let mut grid = Grid::new_from_string(inputstring, false);
    let directions = [
        Direction::Down,
        Direction::Up,
        Direction::Left,
        Direction::Right,
        Direction::DiagonalDownLeft,
        Direction::DiagonalDownRight,
        Direction::DiagonalUpLeft,
        Direction::DiagonalUpRight
        ];

    let points_to_check: Vec<(&Point,&char)> = grid.points.iter().filter(|&point| *point.1 == '@').collect();

    let accessable_points : Vec<Point>= points_to_check.iter()
        // total count of all
        .filter_map(|(point, _character)| {
            let mut adjacent_points = 0;
            for direction in directions {
                if let Some(adjacent_point) = grid.points.get(&point.get_point_in_direction(direction)) {
                    if *adjacent_point == '@' {
                    adjacent_points += 1      
                    }      
                }
                
            }
            if adjacent_points < 4 {
                Some(**point)
            } else {
                None
            }
         }
    ).collect();

    let result = accessable_points.iter().count() as u64;

    match fs::write(Path::new("output/2025-4.txt"), grid.get_grid_as_table()) {
        Ok(saved_bytes) => println!("Successfyllt saved the grid in output/ {:?}", saved_bytes),
        Err(err) => println!("An error occured saving grid {err}")
    };

    accessable_points.iter().for_each(|point| grid.change_cell_data_no_return(&point, 'x'));

/*     for point in &accessable_points {
        grid.change_cell_data(&point, 'x');
    } */

    match fs::write(Path::new("output/2025-4-final.txt"), grid.get_grid_as_table()) {
        Ok(saved_bytes) => println!("Successfyllt saved the grid in output/ {:?}", saved_bytes),
        Err(err) => println!("An error occured saving grid {err}")
    };

    //println!("Number of accessable tp: {}", accessable_points.count());
    return result;
}

fn second_part(inputstring: &str) -> u64 {
    // hent alle @ posisjoner, sjekk om accessable, -> fjern fra grid. repeat.
    let mut grid = Grid::new_from_string(inputstring, false);
    let directions = [
        Direction::Down,
        Direction::Up,
        Direction::Left,
        Direction::Right,
        Direction::DiagonalDownLeft,
        Direction::DiagonalDownRight,
        Direction::DiagonalUpLeft,
        Direction::DiagonalUpRight
        ];
    let mut _keep_checking = true;
    let mut removed_paper_rolls: u64 = 0;

    while _keep_checking {
        let points_to_check: Vec<(&Point,&char)> = grid.points.iter().filter(|&point| *point.1 == '@').collect();
        
        if points_to_check.is_empty() {
            _keep_checking = false;
            break
        }

        if !points_to_check.is_empty() {
            let accessable_points : Vec<Point>= points_to_check.iter()
            // total count of all
            .filter_map(|(point, _character)| {
                let mut adjacent_points = 0;
                for direction in directions {
                    if let Some(adjacent_point) = grid.points.get(&point.get_point_in_direction(direction)) {
                        if *adjacent_point == '@' {
                        adjacent_points += 1      
                        }      
                    }
                }
                if adjacent_points < 4 {
                    Some(**point)
                } else {
                    None
                }
            }
            ).collect();

            if accessable_points.is_empty() {
                _keep_checking = false;
                break
            }

            removed_paper_rolls += accessable_points.iter().count() as u64;

            accessable_points.iter().for_each(|point| grid.change_cell_data_no_return(&point, 'x'));

        }
}


    return removed_paper_rolls;
}
fn main(){
    let test_string = "..@@.@@@@.\n@@@.@.@.@@\n@@@@@.@.@@\n@.@@@@..@.\n@@.@@@@.@@\n.@@@@@@@.@\n.@.@.@.@@@\n@.@@@.@@@@\n.@@@@@@@@.\n@.@.@@@.@.";

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
        assert_eq!(first_part(&file_content), 1428);
    }

    #[test]
    fn second_part_is_correct() {
        let file_content = fs::read_to_string(Path::new(INPUT_FILE_PATH)).expect("Could not load file");
        assert_eq!(second_part(&file_content), 8936);
}

}