use std::{cmp::{Ordering, max}, collections::BTreeMap, fmt::Display, ops::{Add, AddAssign, Mul, Sub}};

#[derive(Debug,PartialEq,Eq,Clone, Copy)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
    DiagonalUpRight,
    DiagonalDownRight,
    DiagonalUpLeft,
    DiagonalDownLeft
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Point {
    pub x: i32,
    pub y: i32
}

impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Point => x: {} y: {}", self.x,self.y)
    }
}

impl Add for Point {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y
        }
    }
}

impl AddAssign for Point {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl AddAssign<(i32,i32)> for Point {
    fn add_assign(&mut self, tuple: (i32,i32)) {
        self.x += tuple.0;
        self.y += tuple.1;
    }
}

impl Sub for Point {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x - rhs.x,
            y: self.y - rhs.y
        }
    }
}

impl Mul for Point {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x * rhs.x,
            y: self.y * rhs.y
        }
    }
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// Standard ordering sorterer x så y (x, y), denne impl sorterer på y, så x for å gi en bedre pretty print
impl Ord for Point {
    fn cmp(&self, other: &Self) -> Ordering {
        let order = self.y.cmp(&other.y);
        match order {
            Ordering::Equal => self.x.cmp(&other.x),
            _ => order,
        }
    }
}

impl Point {
    pub fn get_point_in_direction(&self, direction: Direction) -> Point {
        match direction {
            Direction::Up                => Point { x: self.x, y: self.y-1 },
            Direction::Down              => Point { x: self.x, y: self.y+1 },            
            Direction::Right             => Point { x: self.x+1, y: self.y },
            Direction::Left              => Point { x: self.x-1, y: self.y },
            Direction::DiagonalDownLeft  => Point { x: self.x-1, y: self.y+1 },
            Direction::DiagonalDownRight => Point { x: self.x+1, y: self.y+1 },
            Direction::DiagonalUpLeft    => Point { x: self.x -1, y: self.y -1 },
            Direction::DiagonalUpRight   => Point { x: self.x+1, y: self.y -1 }            
        }
    }

    pub fn get_points_in_direction(&self, direction: Direction, num_points: i32) -> Vec<Point> {
        let mut result: Vec<Point> = Vec::new();
        
        for index in 0..num_points {
            match direction {
                Direction::Up                => result.push(Point { x: self.x, y: self.y-index }),
                Direction::Down              => result.push(Point { x: self.x, y: self.y+index }),            
                Direction::Right             => result.push(Point { x: self.x+index, y: self.y }),
                Direction::Left              => result.push(Point { x: self.x-index, y: self.y }),
                Direction::DiagonalDownLeft  => result.push(Point { x: self.x-index, y: self.y+index }),
                Direction::DiagonalDownRight => result.push(Point { x: self.x+index, y: self.y+index }),
                Direction::DiagonalUpLeft    => result.push(Point { x: self.x -index, y: self.y -index }),
                Direction::DiagonalUpRight   => result.push(Point { x: self.x+index, y: self.y -index })            
            }
        }

        return result;
    }

    pub fn in_bounds(&self, (min_x,max_x) : (i32,i32), (min_y, max_y) : (i32,i32)) -> bool {
        return (self.x >= min_x) && (self.x<max_x) && (self.y >= min_y) && (self.y < max_y);
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Grid { //Grid<T>
    pub width: i32,
    pub height: i32,
    pub points: BTreeMap<Point,char> // vurdere å endre char til en generisk datatype
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Grid size: w={} h={}. \nGrid elements:\n{:?}", self.width,self.height,self.points)
    }
}

impl Grid {
    pub fn new_from_string(inputstring: &str, trim_whitespace: bool) -> Grid {
        let row_width: usize = inputstring.lines().nth(0).unwrap_or("").chars().count();
        let row_height: usize = inputstring.lines().count();
        let mut result: BTreeMap<Point,char> = BTreeMap::new();

        for (y_pos, row_line) in inputstring.lines().enumerate() {
            let row_line_trimmed = if trim_whitespace{row_line.split_ascii_whitespace().collect()}else{row_line.to_string()};
            for (x_pos, character) in row_line_trimmed.char_indices() {
                result.insert(Point { x: x_pos as i32, y: y_pos as i32 }, character);
            }
        }
        Self { 
            width: row_width as i32, 
            height: row_height as i32, 
            points: result }
    }

    pub fn get_pretty_print(&self) -> String {
        let mut result = String::new();
        let mut last_point = Point {x:0,y:0};

        for point in &self.points {
            if last_point.y != point.0.y {
                result.push_str("\n");
            }
            result.push_str(&format!("{}", point.1));
            last_point = *point.0;
        }

        return result;
    }

    pub fn get_grid_as_table(&self) -> String {
        let mut result: String = String::new();
        
        let longest_point = self.points.iter().max_by_key(|(point, _)| max(point.x.to_string().len(), point.y.to_string().len())).unwrap(); // i could also check data when the data is made generic
        let highest_number_length = max(longest_point.0.x.to_string().len(), longest_point.0.y.to_string().len());
        println!("highest_number_length: {}, longest_point: {:?}", highest_number_length, longest_point);
        
        let table_title_text = "DATA";
        let column_header_title = "x/y";

        let column_width: usize = max(column_header_title.len()+2, highest_number_length);

        let column_divider = format!("{}+","-".repeat(column_width));
        let horizontal_divider = format!("+{}+{}","-".repeat(column_width),column_divider.repeat(self.width as usize));

        let table_width = horizontal_divider.len();
        let table_title_spaces = table_width - (table_title_text.len()+2);
        let title = format!("{} {} {}", "#".repeat(table_title_spaces/2), table_title_text, "#".repeat(table_title_spaces - (table_title_spaces/2)));

        
        let vertical_divider = "|";

        // TITLE
        result.push_str(&title);
        result.push_str(&format!("\n{horizontal_divider}\n"));
        
        // HEADER ROW title + index

        result.push_str(&format!("{}", &vertical_divider));
        result.push_str(&format!(" {column_header_title} "));
        result.push_str(&format!("{vertical_divider}"));

        for x_pos in 0..self.width {
            let left_spaces = (column_width - x_pos.to_string().len())/2;
            result.push_str(&format!("{}{x_pos}{}", " ".repeat(left_spaces), " ".repeat(column_width-x_pos.to_string().len()-left_spaces)));
            result.push_str(&format!("{vertical_divider}"));

        }

        // DATA ROW
        for y_pos in 0..self.height{
            let row: Vec<(Point, char)> = self.points.clone()
            .into_iter()
            .filter_map(|(point,data)| {
                if point.y == y_pos {
                    Some((point,data))
                } else{
                    None
                }})
            .collect();

            result.push_str(&format!("\n{horizontal_divider}"));

            let mut spaces = column_width-y_pos.to_string().len();
            let first_row = format!("{vertical_divider}{}{y_pos}{}{vertical_divider}", " ".repeat(spaces/2), " ".repeat(spaces-(spaces/2)));
            result.push_str(&format!("\n{first_row}"));

            for (_, data) in row {
                spaces = column_width-data.to_string().len();
                result.push_str(&format!("{}{}{}{vertical_divider}", " ".repeat(spaces/2), data ," ".repeat(spaces-(spaces/2))));
            }
        }
        result.push_str(&format!("\n{horizontal_divider}"));

        return result;
    }

    pub fn change_cell_data(&mut self, point: &Point, data: char) -> Option<char>{
        return self.points.insert(*point, data);
    }

    pub fn are_points_inbounds(&self, points: Vec<Point>) -> bool {
        let result: Vec<Point> = points.iter().filter_map(|point| {
            if !self.points.contains_key(point) {
                Some(*point)
            } else {
                None
            }

        }).collect();

        if !result.is_empty() {println!("Result: {:?}", result);};
        return result.is_empty();
    }

    pub fn get_string_in_direction(&self, point: Point, num_points: i32, direction: Direction) -> Result<String, String> {
        let mut result: String = String::new();
        
        for index in 0..num_points {
                let next_point = match direction {
                    Direction::Up                => Point { x: point.x, y: point.y-index },
                    Direction::Down              => Point { x: point.x, y: point.y+index },   
                    Direction::Right             => Point { x: point.x+index, y: point.y },
                    Direction::Left              => Point { x: point.x-index, y: point.y },
                    Direction::DiagonalDownLeft  => Point { x: point.x-index, y: point.y+index },
                    Direction::DiagonalDownRight => Point { x: point.x+index, y: point.y+index },
                    Direction::DiagonalUpLeft    => Point { x: point.x -index, y: point.y -index },
                    Direction::DiagonalUpRight   => Point { x: point.x+index, y: point.y -index }            
                
                };

                if !self.points.contains_key(&next_point) {
                    return Err(format!("{} is out of bounds. Startpos was: {}. index: {}", &next_point, &point, &index))
                }

                result.push(*self.points.get(&next_point).expect("Could not get the data from the next point"));
        }

        return Ok(result);

    }
}