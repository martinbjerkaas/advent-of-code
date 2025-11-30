use aoc::*;

#[test]
fn test_initialize_grid_from_string(){
    let test_string = "X..X..X\n..X..X.\nX.X.X.X\n...X.N.";    
    
    let test_grid = Grid::new_from_string(test_string, false);

    assert_eq!(*test_grid.points.get(&Point { x: 3, y: 0 }).expect("Could not get point"),'X');
    assert_eq!(*test_grid.points.get(&Point { x: 5, y: 3 }).expect("Could not get point"),'N');
    assert_eq!(test_grid.height, 4);
    assert_eq!(test_grid.width,7);
}

#[test]
fn test_all_directions_from_point(){
    let test_string = "X..X..X\n..XMUE.\nX.XLXRX\n...TDN.";    
    //     [0][1][2][3][4][5][6]
    // 0   [X][.][.][X][.][.][X]
    // 1   [.][.][X][M][U][E][.]
    // 2   [X][.][X][L][X][R][X]
    // 3   [.][.][.][T][D][N][.]

    
    let test_grid = Grid::new_from_string(test_string, false);

    let test_point = Point { x: 4, y: 2 };

    assert_eq!(*test_grid.points.get(&test_point).expect("Could not get point"),'X');
    assert_eq!(*test_grid.points.get(&test_point.get_point_in_direction(Direction::Up)).expect("Could not get point with direction up"), 'U');
    assert_eq!(*test_grid.points.get(&test_point.get_point_in_direction(Direction::Right)).expect("Could not get point with direction up"), 'R');
    assert_eq!(*test_grid.points.get(&test_point.get_point_in_direction(Direction::Down)).expect("Could not get point with direction up"), 'D');
    assert_eq!(*test_grid.points.get(&test_point.get_point_in_direction(Direction::Left)).expect("Could not get point with direction up"), 'L');
    assert_eq!(*test_grid.points.get(&test_point.get_point_in_direction(Direction::DiagonalDownRight)).expect("Could not get point"),'N');
    assert_eq!(*test_grid.points.get(&test_point.get_point_in_direction(Direction::DiagonalDownLeft)).expect("Could not get point"),'T');
    assert_eq!(*test_grid.points.get(&test_point.get_point_in_direction(Direction::DiagonalUpRight)).expect("Could not get point"),'E');
    assert_eq!(*test_grid.points.get(&test_point.get_point_in_direction(Direction::DiagonalUpLeft)).expect("Could not get point"),'M');
    

}

#[test]
fn test_print_as_table(){
    let test_string = "X..X..X\n..XMUE.\nX.XLXRX\n...TDN.";  
    let test_table = "##################### DATA ######################\n+-----+-----+-----+-----+-----+-----+-----+-----+\n| x/y |  0  |  1  |  2  |  3  |  4  |  5  |  6  |\n+-----+-----+-----+-----+-----+-----+-----+-----+\n|  0  |  X  |  .  |  .  |  X  |  .  |  .  |  X  |\n+-----+-----+-----+-----+-----+-----+-----+-----+\n|  1  |  .  |  .  |  X  |  M  |  U  |  E  |  .  |\n+-----+-----+-----+-----+-----+-----+-----+-----+\n|  2  |  X  |  .  |  X  |  L  |  X  |  R  |  X  |\n+-----+-----+-----+-----+-----+-----+-----+-----+\n|  3  |  .  |  .  |  .  |  T  |  D  |  N  |  .  |\n+-----+-----+-----+-----+-----+-----+-----+-----+";

    let grid = Grid::new_from_string(test_string, false);

    println!("{}", grid.get_grid_as_table());
    assert_eq!(grid.get_grid_as_table(), test_table);
}

#[test]
fn test_input_with_whitespace(){
    let test_string = " ..X..X\n..XM E.\nX.X XRX\n...TDN ";
    println!("{test_string}");
}