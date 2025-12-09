use std::{collections::{BTreeMap, HashMap, HashSet}, fs, path::Path};
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

    //let _ = fs::write(Path::new("output/2025-7-pretty.txt"), grid.get_pretty_print_clean('.'));
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

/*     // tar utgangspunkt i at alle beams i bunnen er et unikt startpunkt.
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
 */ 

     for beam in beams {
        if beam != start {
            grid.change_cell_data_no_return(&beam, '|');
        }
    }

    // Point : [Point,Point]
    let directions = [Direction::DiagonalDownLeft, Direction::Down, Direction::DiagonalDownRight];
    let mut graph: BTreeMap<Point, Vec<Point>> = BTreeMap::new();

    for y in 0..grid.height {
        let all_pipes = grid.points.iter().filter(|(point, character)| (point.y==y) && (**character=='|'));
        println!("Pipes at y: {}", y);
        for (pipepoint, pipechar) in all_pipes {
            println!("{:?} - {:?}", &pipepoint, &pipechar);
            for direction in directions {
                let next_char: Option<char> = match direction {
                    Direction::DiagonalDownLeft  => grid.points.get(&pipepoint.get_point_in_direction(direction)).copied(),
                    Direction::Down              => grid.points.get(&pipepoint.get_point_in_direction(direction)).copied(),
                    Direction::DiagonalDownRight => grid.points.get(&pipepoint.get_point_in_direction(direction)).copied(),
                    _ => Some('U')
                };

                if let Some(pipe_char) = next_char {
                    if pipe_char == '|' {
                    graph.entry(*pipepoint).or_insert_with(Vec::new).push(pipepoint.get_point_in_direction(direction));
                }
            }
            }
        }
    }

    for (node, childs) in &graph {

        println!("Node: {:?} Childs: {:?}", node, childs);
    }
    
    return splits
}

fn second_part_two(inputstring: &str) {
    let mut grid = Grid::new_from_string(inputstring, false);
    let mut nodes: BTreeMap<Point, Vec<Point>> = BTreeMap::new();
    let mut splits = 0;
    let mut current_node = *grid.points.iter().find(|(_, character)| **character == 'S').expect("Could not find start pos").0;
    println!("Startnode: {:?}", current_node);

    // spare_forrige y sine nodes.
    
    let mut children: Vec<Point> = Vec::new(); 
    let mut pending_nodes : Vec<Point> = Vec::new();
    let mut next_nodes: Vec<Point> = Vec::new();
    next_nodes.push(current_node);

    let mut total_children = 0;

    for row_index in 0..grid.height {
        while next_nodes.len() > 0 {
            //println!("Next nodes: {:?} Len: {}", &next_nodes, next_nodes.len());
            let node = next_nodes.pop().unwrap();
            
            println!("Node: {:?} Len: {}", node, next_nodes.len());

            match grid.points.get(&node.get_point_in_direction(Direction::Down)) {
                Some('^') => {
                    children.push(node.get_point_in_direction(Direction::DiagonalDownLeft));
                    children.push(node.get_point_in_direction(Direction::DiagonalDownRight));
                    splits+=1;
                },
                Some('.') => {
                    children.push(node.get_point_in_direction(Direction::Down));
                },
                _ => (),
                None => ()
            }
            nodes.insert(node, children.clone());
            pending_nodes.append(&mut children);
            
            
            //println!("Nodes: {:?}   Children: {:?}   Pending: {:?}", nodes, children, pending_nodes);
            children.clear();
        }

        println!(" ----- next node loop finished -----");

        next_nodes = pending_nodes.clone();
        pending_nodes.clear();
    }

    println!("Nodes: {:?}", nodes);
    println!("Total splits: {}", nodes.iter().filter(|x|x.1.len()>1).map(|x| x.1.len() as u16).sum::<u16>())
    
}
fn main(){
    let test_string = ".......S.......\n...............\n.......^.......\n...............\n......^.^......\n...............\n.....^.^.^.....\n...............\n....^.^...^....\n...............\n...^.^...^.^...\n...............\n..^...^.....^..\n...............\n.^.^.^.^.^...^.\n...............";

    //println!("First part example: {:?}", first_part(test_string));
    //println!("Second part example: {:?}", second_part(test_string));
    println!("Second part 2 example: {:?}", second_part_two(test_string));

    match fs::read_to_string(Path::new(INPUT_FILE_PATH)) {
        Ok(file_content) => {
            //println!("First part file input: {:?}", first_part(&file_content));
            //println!("Second part file input: {:?}", second_part_two(&file_content));
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