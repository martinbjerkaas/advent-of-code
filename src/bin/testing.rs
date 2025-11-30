use std::{fs, path::Path};

use aoc::*;

fn main() {
    let _test_grid_string = "**********\n**********\n**********\n**********\n**********\n**********\n";
    let _testmas = "MMMSXXMASM\nMSAMXMSMSA\nAMXSXMAAMM\nMSAMASMSMX\nXMASAMXAMM\nXXAMMXXAMA\nSMSMSASXSS\nSAXAMASAAA\nMAMMMXMMMM\nMXMXAXMASX\nMXMXAXMASX";

    //let grid = Grid::new_from_string(testmas, false);
    let grid = Grid::new_from_string(&fs::read_to_string(Path::new("data/2024-4.txt")).expect("Could not load file"), false);
    println!("{}", grid);
    //grid.print_grid();

    let startpoint = Point {x:4, y:0};
    let (bounds_x, bounds_y) = ((0,grid.width), (0,grid.height));

    println!("Startpoint is in grid: {}", startpoint.in_bounds(bounds_x,bounds_y));

    let diagonal_text: String = startpoint.get_points_in_direction(Direction::DiagonalDownRight, 5)
    .iter()
    .map(|x|grid.points.get(x).expect("Could not get the point from the grid."))
    .collect();

    let all_x : Vec<Point>= grid.points.iter().filter_map(|(point, character)| {
        match character {
            'M' => Some(*point),
            _ => None
        }
    }).collect();

    println!("{diagonal_text}");
    println!("{:?}",all_x);
/*     while startpoint.x < grid.width && grid.point_is_inbounds(&startpoint){
        println!("Changing {:?} at {} with {}", grid.change_cell_data(&startpoint, 'X'), &startpoint, 'X');
        startpoint.x +=1;
    } */

    println!("{}", grid.get_pretty_print());
    print!("\n");

    //print!("\n{}", grid.get_grid_as_table());

    println!("Created output: {}", fs::write(Path::new("output/test.txt"), grid.get_grid_as_table()).is_ok());

    let ws_test_string = " ..X..X\n..XMUE.\nX.XLXRX\n...TDN ";
    let ws_test_grid = Grid::new_from_string(ws_test_string, true);

    println!("{}", ws_test_grid.get_grid_as_table());

}