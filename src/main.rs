use std::cmp::Ordering;
use std::fmt;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::env;
use std::collections::HashSet;

#[derive(Default)]
struct MapMatrix {
    matrix: Vec<Vec<char>>,
}

impl fmt::Debug for MapMatrix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        let height = self.matrix.len();
        let width = self.matrix[0].len();
        for i in 0..height {
            for j in 0..width {
                let val = self.matrix[i][j];
                write!(f, "{val} ");
            }
            writeln!(f);
        }
        Ok(())
    }
}

trait LoadInput {
    fn push(&mut self, the_vec_to_add: Vec<char> );
}

trait Obstacle {
    fn set_char(&mut self, x: usize, y: usize, the_char: char);
    fn get_char(&self, x: usize, y: usize) -> char;
    }

impl Obstacle for MapMatrix {
    fn set_char(&mut self, x: usize, y: usize, the_char: char) {
        self.matrix[x][y] = the_char;
    }
    fn get_char(&self, x: usize, y: usize) -> char {
        return self.matrix[x][y];
    }

}

impl LoadInput for MapMatrix {
    fn push(&mut self, the_vec_to_add: Vec<char> ) {
        self.matrix.push(the_vec_to_add);
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

fn main() {
    println!("Hello, aoc_2024_6!");

    if let Ok(lines) = read_lines("./src/input.txt") {

        let mut map_matrix = MapMatrix::new();
 
        // Consumes the iterator, returns an ( Optional) String
        for line in lines.flatten() {
            let characters:Vec<char> = line.chars().collect();
            map_matrix.push(characters);
        }

        println!("\nmap_matrix: \n{:?}", map_matrix);

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