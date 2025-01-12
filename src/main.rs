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

enum GuardResult {
    MovedNormally,
    MovedOffMap,
    InLoop,
    GuardBlocked,
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

    // As termination condition, returns same position and orientation if we cannot move or we went off the map.
    fn move_guard(&mut self, map_matrix: &MapMatrix) -> GuardResult {
        let (new_x, new_y) = self.advance_coords();

        if map_matrix.in_bounds(new_x, new_y) {
            // new coordinates are within the map boundaries, see if we can move there.
            if ! map_matrix.is_obstacle(new_x, new_y) {
                self.x = new_x;
                self.y = new_y;
                return GuardResult::MovedNormally;
            } else if let Some(orientation) =  is_guard(map_matrix.get_char(new_x, new_y)) {
                if orientation == self.orientation {
                    return GuardResult::InLoop;
                } else {
                    // We're passing over an already checked square but in a different direction.
                    self.x = new_x;
                    self.y = new_y;
                    return GuardResult::MovedNormally;
                }
            } else {
                // We are in map but would run into an obstacle, see if changing orientation of original and advancing works.
                let mut tentative_orientation = self.orientation;
                for _ in 0..3 {
                    tentative_orientation = tentative_orientation.rotate();
                    let mut tentative_new_guard = self.clone();
                    tentative_new_guard.orientation = tentative_orientation;
                    let ( new_x, new_y ) = tentative_new_guard.advance_coords();
                    tentative_new_guard.x = new_x;
                    tentative_new_guard.y = new_y;
                    if map_matrix.in_bounds(tentative_new_guard.x, tentative_new_guard.y) && ! map_matrix.is_obstacle(tentative_new_guard.x, tentative_new_guard.y)  {
                        self.x = new_x;
                        self.y = new_y;
                        self.orientation = tentative_new_guard.orientation;
                        return GuardResult::MovedNormally;
                    }
                }
                // We tried all orientations and none worked, so return Guard unchanged and indication to stop.
                return GuardResult::GuardBlocked;
            }
        } else {
            // We went off the map, so return Guard unchanged and indication to stop.
            return GuardResult::MovedOffMap;
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
    fn is_guard(&self, x: usize, y: usize ) -> bool;
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
        self.get_char(x, y) == '#' || self.get_char(x, y) == 'O'
    }
    fn is_guard(&self, x: usize, y: usize ) -> bool {
        is_guard(self.get_char(x, y)).is_some()
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
        for r in 0 .. self.matrix.len() {
            matrix.push( self.matrix[r].clone() );
        }
        Self { matrix }
    }    
}

fn simulate_guard(map_matrix: &mut MapMatrix) -> GuardResult {

    let mut the_guard = map_matrix.find_guard().unwrap();

    let mut loop_counter: usize = 0;
    let max_loop_iterations = 100 * map_matrix.height() * map_matrix.width();
    loop {
        //println!("Analyzing guard: {the_guard}");
        // Add your analysis logic here
        match the_guard.move_guard(&map_matrix) {
            GuardResult::MovedNormally => {
                map_matrix.set_char(the_guard.x, the_guard.y, the_guard.orientation.to_char());
            }
            GuardResult::MovedOffMap => return GuardResult::MovedOffMap,
            GuardResult::InLoop => return GuardResult::InLoop,
            GuardResult::GuardBlocked => return GuardResult::GuardBlocked,
        }

        loop_counter += 1;
        if loop_counter > max_loop_iterations {
            return GuardResult::InLoop;
        }
        //println!("\nmap_matrix: \n{}", map_matrix);
    }

}




fn main() {
    println!("Hello, aoc_2024_6!");

    if let Ok(lines) = read_lines("./src/input.txt") {

        let mut input_map_matrix = MapMatrix::new();
 
        // Consumes the iterator, returns an ( Optional) String
        for line in lines.flatten() {
            let characters:Vec<char> = line.chars().collect();
            input_map_matrix.push(characters);
        }

        println!("\ninput_map_matrix: \n{}", input_map_matrix);

        // Part 1
        let mut cloned_map_matrix = input_map_matrix.clone();
        match simulate_guard(&mut cloned_map_matrix) {
            GuardResult::MovedOffMap => println!("Guard moved off the map."),
            GuardResult::InLoop => println!("Guard is in a loop."),
            GuardResult::GuardBlocked => println!("NOT EXPECTED given problem assumptions: Guard is blocked."),
            GuardResult::MovedNormally => println!("NOT EXPECTED because of loop in simulate_guard: Guard moved normally."),
        }

        println!("\nFINAL part 1 map_matrix: \n{}", cloned_map_matrix);
        println!("Part 1: Number of guard visited spaces: {}", cloned_map_matrix.count_guard_spaces());

        // Part 2
        let mut loops_found : usize = 0;
        for block_y in 0..input_map_matrix.height() {
            for block_x in 0..input_map_matrix.width() {

                let mut cloned_map_matrix = input_map_matrix.clone();

                if ! cloned_map_matrix.is_obstacle(block_x, block_y) && ! cloned_map_matrix.is_guard(block_x, block_y) {
                    // Set a new temporary obstacle.
                    cloned_map_matrix.set_char(block_x, block_y, 'O');

                    //println!("Checking for loop start)\n{}", cloned_map_matrix);
                    match simulate_guard(&mut cloned_map_matrix) {
                        GuardResult::MovedOffMap => (),
                        GuardResult::InLoop => {
                            loops_found += 1;
                            //println!("Loop found at ({}, {})\n {}", block_x, block_y, cloned_map_matrix);
                        },
                        GuardResult::GuardBlocked => println!("NOT EXPECTED given problem assumptions: Guard is blocked."),
                        GuardResult::MovedNormally => println!("NOT EXPECTED because of loop in simulate_guard: Guard moved normally."),
                    }
                    //println!("Checking for loop end)\n{}", cloned_map_matrix);

                }
            }
        }


        println!("Part 2: Number of loop possible guard loops: {loops_found}");
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