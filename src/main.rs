use std::fmt;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Default)]
struct MapMatrix {
    matrix: Vec<Vec<char>>,
    
}

#[derive(Clone, Copy, Eq, PartialEq)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn from_char(c: char) -> Option<Self> {
        match c {
            '^' => Some(Direction::North),
            '>' => Some(Direction::East),
            'v' => Some(Direction::South),
            '<' => Some(Direction::West),
            _ => None,
        }
    }

    fn to_char(&self) -> char {
        match self {
            Direction::North => '^',
            Direction::East => '>',
            Direction::South => 'v',
            Direction::West => '<',
        }
    }
}

impl Direction {
    fn rotate(&self) -> Direction {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }
}
impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_char())
    }
}

#[derive(Copy, Eq, Clone, PartialEq)]
struct Guard {
    x: usize,
    y: usize,
    orientation: Direction,
}
impl fmt::Display for Guard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Guard at ({}, {}), facing '{}'", self.x, self.y, self.orientation)
    }
}

impl Guard {
    fn advance_coords(&self) -> (usize, usize){
        let (new_x, new_y) = match self.orientation {
            Direction::South => (self.x, self.y + 1),
            Direction::North => (self.x, self.y.wrapping_sub(1)),
            Direction::West => (self.x.wrapping_sub(1), self.y),
            Direction::East => (self.x + 1, self.y),
        };
        (new_x, new_y)
    }

    fn rotate(&mut self) {
        self.orientation = self.orientation.rotate();
    }

    // As termination condition, returns same position and orientation if we cannot move or we went off the map.
    fn move_guard(self, map_matrix: &MapMatrix) -> Self {
        let (new_x, new_y) = self.advance_coords();

        if map_matrix.in_bounds(new_x, new_y) {
            // new coordinates are within the map boundaries, see if we can move there.
            if ! map_matrix.is_obstacle(new_x, new_y) {
                // Return new position and orientation.
                Guard { x: new_x, y: new_y, orientation: self.orientation }
            } else {
                // We are in map but would ry into an obstacle, so changing orientation of original and advancing works.
                let mut tentative_new_guard = self.clone();
                for _ in 0..3 {
                    tentative_new_guard.rotate();
                    let ( new_x, new_y ) = tentative_new_guard.advance_coords();
                    tentative_new_guard.x = new_x;
                    tentative_new_guard.y = new_y;
                    if map_matrix.in_bounds(tentative_new_guard.x, tentative_new_guard.y) && ! map_matrix.is_obstacle(tentative_new_guard.x, tentative_new_guard.y)  {
                        return tentative_new_guard;
                    }
                }
                // We tried all orientations and none worked, so return Guard unchanged and indication to stop.
                self
            }
        } else {
            // We went off the map, so return Guard unchanged and indication to stop.
            self
        }
    }
}



impl fmt::Display for MapMatrix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        let height = self.matrix.len();
        let width = self.matrix[0].len();
        for y in 0..height {
            for x in 0..width {
                let val = self.matrix[y][x];
                if let Err(e) = write!(f, "{val} ") {
                    panic!("Writing error: {e}");
                }
            }
            if let Err(e) = writeln!(f) {
                panic!("Writing error: {e}");
            }
        }
        Ok(())
    }
}

trait LoadInput {
    fn push(&mut self, the_vec_to_add: Vec<char> );
    fn find_guard(&self) -> Result<Guard, &'static str>;
}

trait Obstacle {
    fn set_char(&mut self, x: usize, y: usize, the_char: char);
    fn get_char(&self, x: usize, y: usize) -> char;
    fn height(&self) -> usize;
    fn width(&self) -> usize;
    fn in_bounds(&self, x: usize, y: usize) -> bool;
    fn is_obstacle(&self, x: usize, y: usize ) -> bool;
    }

impl Obstacle for MapMatrix {
    fn set_char(&mut self, x: usize, y: usize, the_char: char) {
        self.matrix[y][x] = the_char;
    }
    fn get_char(&self, x: usize, y: usize) -> char {
        return self.matrix[y][x];
    }
    fn height(&self) -> usize {
        return self.matrix.len();
    }
    fn width(&self) -> usize {
        if self.matrix.len() > 0 {
            // Assumes uniform.
            return self.matrix[0].len();
        }
        return 0;
    }
    fn in_bounds(&self, x: usize, y: usize) -> bool {
        x < self.width() && y < self.height()
    }
    fn is_obstacle(&self, x: usize, y: usize ) -> bool {
        self.get_char(x, y) == '#'
    }
}

impl MapMatrix {
    fn count_guard_spaces(&self) -> usize {
        let mut count = 0;
        for row in &self.matrix {
            for &cell in row {
                if is_guard(cell).is_some() {
                    count += 1;
                }
            }
        }
        count
    }
}

fn is_guard(the_char: char) -> Option<Direction> {
    Direction::from_char(the_char)
}

impl LoadInput for MapMatrix {
    fn push(&mut self, the_vec_to_add: Vec<char> ) {
        self.matrix.push(the_vec_to_add);
    }
    fn find_guard(&self) -> Result<Guard, &'static str> {
        for x in 0.. self.width() {
            for y in 0..self.height() {
                let the_char = self.get_char(x, y);
                if let Some(the_orientation) = is_guard( the_char ) {
                    return Ok(Guard { x: x, y : y, orientation: the_orientation})
                }
            }
        }
        return Err("No guard found");
    }
}

impl MapMatrix {
    fn new() -> Self {
        let matrix: Vec<Vec<char>> = Vec::new();
        Self { matrix }
    }
}

impl Clone for MapMatrix {
    fn clone(&self) -> Self {
        let mut matrix = Vec::new();
        for r in 1 .. self.matrix.len() {
            matrix.push( self.matrix[r].clone() );
        }
        Self { matrix }
    }    
}

enum GuardResult {
    MovedOffMap,
    InLoop,
}

fn simulate_guard(map_matrix: &mut MapMatrix) -> GuardResult {

    let mut the_guard = map_matrix.find_guard().unwrap();

    let max_allowed_iterations = map_matrix.height() * map_matrix.width();
    let mut iterations: usize = 0;
    loop {
        //println!("Analyzing guard: {the_guard}");
        // Add your analysis logic here
        let new_guard = the_guard.move_guard(&map_matrix);
        if new_guard != the_guard {
            //println!("Guard moved from {} to {}", the_guard, new_guard);
            // Mark where we visited.
            map_matrix.set_char(new_guard.x, new_guard.y, new_guard.orientation.to_char());
            iterations += 1;
            if iterations > max_allowed_iterations {
                //println!("Guard did not stop after {} iterations", max_allowed_iterations);
                return GuardResult::InLoop;
            }                
        } else {
            //println!("Guard did not move");
            return GuardResult::MovedOffMap;
        }
        the_guard = new_guard;
        //println!("\nmap_matrix: \n{}", map_matrix);
    }

}




fn main() {
    println!("Hello, aoc_2024_6!");

    if let Ok(lines) = read_lines("./src/input.txt") {

        let mut map_matrix = MapMatrix::new();
 
        // Consumes the iterator, returns an ( Optional) String
        for line in lines.flatten() {
            let characters:Vec<char> = line.chars().collect();
            map_matrix.push(characters);
        }

        println!("\nmap_matrix: \n{}", map_matrix);

        match simulate_guard(&mut map_matrix) {
            GuardResult::MovedOffMap => println!("Guard moved off map"),
            GuardResult::InLoop => println!("Guard in loop"),
        }

        println!("\nFINAL map_matrix: \n{}", map_matrix);

        println!("Number of guard visited spaces: {}", map_matrix.count_guard_spaces());
    }
}

// Thanks to https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html
// The output is wrapped in a Result to allow matching on errors.
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}