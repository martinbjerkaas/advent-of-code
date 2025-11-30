/* 
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#... */

use std::{collections::BTreeMap, fs, path::Path, usize};

#[derive(Debug, Clone, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Right,
    Left
}

#[derive(Debug,Clone,PartialEq, Eq)]
struct Guard {
    x: usize,
    y: usize,
    direction: Direction,
    steps: usize
}

impl Guard {
    fn new_from_grid(grid: &mut Grid) -> Guard {
        let guard = grid.cells
                        .iter()
                        .find(|(_pos, guard)| guard.cell_type == CellType::Guard)
                        .expect("Could not find any guards");

        Self { x: guard.0.0, y: guard.0.1, direction: Direction::Up , steps: 0}
    }

    fn move_guard(&mut self, direction: Direction) {
        match direction {
            Direction::Up => self.y -= 1,
            Direction::Down => self.y += 1,
            Direction::Right => self.x +=1,
            Direction::Left => self.x -= 1
        }
        self.steps += 1;
        self.direction = direction;
    }

    fn get_guard_pos(&self) -> (usize,usize) {
        return (self.x,self.y)
    }
}

#[derive(Debug, Clone, Copy,PartialEq, Eq)]
enum CellType {
    Obsticle,
    Floor,
    Guard,
    Path,
    Unknown
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Cell {
    cell_type: CellType
}
#[derive(Debug, Clone, PartialEq, Eq)]
struct Grid {
    width: usize,
    height: usize,
    cells: BTreeMap<(usize,usize), Cell>
}

impl Grid {
    fn new_from_string(inputstring: &str) -> Grid {
        let mut parsed_cells: BTreeMap<(usize, usize), Cell> = BTreeMap::new();

        for (row_index, row) in inputstring.lines().enumerate() {
            for (column_index, column_char) in row.chars().into_iter().enumerate(){
                let cell_type = match column_char {
                    '#' => CellType::Obsticle,
                    '.' => CellType::Floor,
                    '^' => CellType::Guard,
                    'X' => CellType::Path,
                    _   => CellType::Unknown
                };

                parsed_cells.insert((column_index,row_index), Cell {cell_type});

            }
        }

        Self { 
            width: inputstring.lines().last().unwrap().chars().count(), 
            height: inputstring.lines().count(), 
            cells: parsed_cells }
    }
}

fn print_current_grid(grid:&Grid) {
    let mut last_row = 0;

    let mut sorted_grid: Vec<((usize, usize), Cell)> = grid.cells.clone().into_iter().collect();
    sorted_grid.sort_by_key(|((first, second), _)| (*second, *first));

    for (pos, cell) in sorted_grid {

        let char_to_print = match cell.cell_type {
            CellType::Floor => '.',
            CellType::Guard => '^',
            CellType::Obsticle => '#',
            CellType::Path => 'X',
            CellType::Unknown => '?'
        };

        if last_row != pos.1 {
            print!("\n");
        }
        
        print!("{}", char_to_print);
        last_row = pos.1;
    }
}

fn simulate_guard_path(guard: &mut Guard, grid: &mut Grid) -> bool {
    grid.cells.get_mut(&guard.get_guard_pos()).expect("Could not find guard pos").cell_type = CellType::Path;

    let next_direction = match guard.direction {
        Direction::Up => {
            if (guard.y as isize - 1) < 0 {
                return false
            } else if grid.cells.get(&(guard.x,guard.y-1)).expect("Could not find cell").cell_type == CellType::Obsticle {
                Direction::Right
            } else {Direction::Up}
        },
        Direction::Right => {
            if (guard.x as isize + 1) == grid.width as isize {
                return false
            } else if grid.cells.get(&(guard.x+1,guard.y)).expect("Could not find cell").cell_type == CellType::Obsticle {
                Direction::Down
            } else {Direction::Right}
        },
        Direction::Down => {
            if (guard.y as isize + 1) == grid.height as isize{
                println!("Out of bounds");
                return false
            } else if grid.cells.get(&(guard.x,guard.y+1)).expect("Could not find cell").cell_type == CellType::Obsticle {
                Direction::Left
            } else {Direction::Down}
        },
        Direction::Left => {
            if (guard.x as isize - 1) < 0 {
                return false
            } else if grid.cells.get(&(guard.x-1,guard.y)).expect("Could not find cell").cell_type == CellType::Obsticle {
                Direction::Up
            } else {Direction::Left}
        },
    };

    guard.move_guard(next_direction);
    grid.cells.get_mut(&guard.get_guard_pos()).expect("Could not find guard pos").cell_type = CellType::Guard;
    return true;

}

fn count_guard_path(grid: &Grid) -> i32 {
    let mut count = 0;
    for (_pos,cell) in &grid.cells {
        if cell.cell_type == CellType::Path {
            count += 1;
        }
    }
    return count;
}

fn main(){
    let _test_string = "....#.....\n.........#\n..........\n..#.......\n.......#..\n..........\n.#..^.....\n........#.\n#.........\n......#...";
    let file_content = fs::read_to_string(Path::new("data/day6.txt")).expect("Could not read file");
    let mut grid = Grid::new_from_string(&file_content);
    let mut guard = Guard::new_from_grid(&mut grid);

    //println!("{:#?}", &grid);
    
    //print_current_grid(&grid);

    while simulate_guard_path(&mut guard, &mut grid) {
       
    }

    print_current_grid(&grid);

    println!("Guard steps: {}", count_guard_path(&grid))
}