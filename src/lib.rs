use std::fs::File;
use std::io::{ BufReader, BufRead };

//mod algorithms;

#[derive(Debug)]
pub struct Config {
    pub file_path: String,
}

impl Config {
    pub fn build(
        mut args: impl Iterator<Item = String>,
    ) -> Result<Config, &'static str> {
        args.next(); // Skip prepiatory args[0]

       let file_path = match args.next() {
           Some(arg) => arg,
           None => return Err("Try giving a file path"),
       };
       
       Ok(Config { file_path })
    }
}

#[derive(Debug)]
pub enum Cell {
    Wall,
    Path,
    Current,
    Visited,
    Solution,
}

#[derive(Debug)]
pub struct Labyrinth {
    pub grid: Vec<Vec<Cell>>,
    pub start: (usize, usize),
    pub goal: (usize, usize),
    pub size: usize,
}

impl Labyrinth {
    pub fn new(args: &Config) -> Self {
        if let Some(file_path) = Some(&args.file_path) {
            Labyrinth::from_file(&file_path)
        } else {
            // Generate a maze if no args given
            todo!();
        }
    }

    fn from_file(path: &str) -> Self {
        let file = File::open(&path).expect("Unable to open file");
        let reader = BufReader::new(file);

        let mut grid = Vec::new();
        let mut start = (0, 0);
        let mut goal = (0, 0);

        for (i, line) in reader.lines().enumerate() {
            let line = line.expect("Unable to read line");
            let mut row = Vec::new();
            for (j, symbol) in line.chars().enumerate() {
                let cell = match symbol {
                    '#' => Cell::Wall,
                    ' ' => Cell::Path,
                    '^' => {
                        start = (i, j);
                        Cell::Path
                    }
                    'E' => {
                        goal = (i, j);
                        Cell::Path
                    }
                    _ => Cell::Wall,
                };
                row.push(cell);
            }
            grid.push(row);
        }

        let size = grid.len();
        Labyrinth {
            size,
            grid,
            start,
            goal,
        }
    }

    pub fn manhattan_distance(&self, pos: (usize, usize)) -> usize {
    (   (pos.0 as isize - self.goal.0 as isize).abs() +
        (pos.1 as isize - self.goal.1 as isize).abs()) as usize
    }

    pub fn display(&self) {
        for row in &self.grid {
            for cell in row {
                let symbol = match cell {
                    Cell::Wall => format!("#"),
                    Cell::Path => " ".to_string(),
                    Cell::Current => format!("^"),
                    Cell::Visited => format!("@"),
                    Cell::Solution => format!("S")
                };
                print!("{}", symbol)
            }
            println!();
        }
    }
}

pub fn welcome() {
    let default = String::from("a basic");
    println!("\n\n\tPentti got stuck in a maze.\n\tFinding a way out with {} algorithm", default);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn failing() {
        assert_eq!(0, 1);
    }
}
