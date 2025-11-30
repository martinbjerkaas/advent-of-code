use aoc::*;

fn first_part(inputstring: &str){
    let grid = Grid::new_from_string(inputstring, true);
    grid.get_pretty_print();

    let mut next_pos = Point {x:0,y:0};
    print!("\n");

    let row_length: usize = grid.points.iter().filter_map(|(point, num)| {
        if point.y == next_pos.y {
            num.to_digit(10)
        } else {
            None
        }
    })
    .collect::<Vec<u32>>()
    .len();

    println!("{:?}", row_length);
    
    while next_pos.in_bounds((0,row_length as i32), (0,1)) {
        println!("Nextpos: {}", &next_pos);
        next_pos = next_pos.get_point_in_direction(Direction::Right);
    }
    
}

fn main(){
    let example_input = "7 6 4 2 1\n1 2 7 8 9\n9 7 6 2 1\n1 3 2 4 5\n8 6 4 4 1\n1 3 6 7 9";
    first_part(example_input);
}

#[cfg(test)]
mod tests {
    //use super::*;

    //#[test]
}