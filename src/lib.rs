/*
Collection of basic public mod's and functionality.
–Jooa Jaakkola
*/

use std::fs::File;
use std::io::{ BufReader, BufRead };

use std::{thread, time::Duration, io::Write};

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

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Cell {
    Wall,
    Path,
    Current,
    Visited,
    Solution,
}

#[derive(Debug)]
pub struct Labyrinth {
    pub size: usize,
    pub grid: Vec<Vec<Cell>>,
    pub start: (usize, usize),
    pub goal: (usize, usize),
    pub width: usize,
    pub height: usize,
    pub up: (isize, isize),
    pub right: (isize, isize),
    pub down: (isize, isize),
    pub left: (isize, isize),
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

    pub fn display(&self) {
        for row in &self.grid {
            for cell in row {
                let symbol = match cell {
                    Cell::Wall => format!("#"),
                    Cell::Path => " ".to_string(),
                    Cell::Current => format!("^"),
                    Cell::Visited => format!("*"),
                    Cell::Solution => format!("E")
                };
                print!("{}", symbol)
            }
            println!();
        }
        println!("Start: {:?}, Goal: {:?}, Width: {:?}, Height: {:?}, Size: {}", 
            self.start, self.goal, self.width, self.height, self.size);
    }

    pub fn is_path(&self) -> Option<usize> {
        let mut solution_path: usize = 0;

        for row in &self.grid {
            for cell in row {
                match cell {
                    Cell::Current | Cell::Visited => {
                        solution_path += 1;
                    },
                    _ => continue,
                }
            }
        }

        Some(solution_path)
    }

    fn manhattan_distance(&self, pos: (usize, usize)) -> usize {
    (   (pos.0 as isize - self.goal.0 as isize).abs() +
        (pos.1 as isize - self.goal.1 as isize).abs()) as usize
    }

    fn from_file(path: &str) -> Self {
        let file = File::open(&path).expect("Unable to open file");
        let reader = BufReader::new(file);

        let mut grid = Vec::new();
        let mut start = (0, 0);
        let mut goal = (0, 0);
        let mut width = 0;

        let up = (0, -1);
        let right = (1, 0);
        let down = (0, 1);
        let left = (-1, 0);

        for (row_i, line) in reader.lines().enumerate() {
            let line = line.expect("Unable to read line");
            let mut row = Vec::new();
            for (cell_j, symbol) in line.chars().enumerate() {
                let cell = match symbol {
                    '#' => Cell::Wall,
                    ' ' => Cell::Path,
                    '^' => {
                        start = (cell_j, row_i);
                        Cell::Current
                    }
                    'E' => {
                        goal = (cell_j, row_i);
                        Cell::Solution
                    }
                    _ => Cell::Visited,
                };
                row.push(cell);
                width = row.len();
            }
            grid.push(row);
        }

        let height = grid.len();
        let size = width * height;

        Labyrinth {
            size,
            grid,
            start,
            goal,
            width,
            height,
            up,
            right,
            down,
            left,
        }
    }
}

pub enum SearchResult {
    Found(Vec<(usize, usize)>),
    NewBound(usize),
}

#[derive(Clone)]
pub struct SearchState {
    path: Vec<(usize, usize)>,
    visited: Vec<Vec<bool>>,
}

fn search(
    labyrinth: &mut Labyrinth,
    g: usize,
    bound: usize,
    state: &mut SearchState,
) -> SearchResult {
    let current = *state.path.last().unwrap();
    let h = labyrinth.manhattan_distance(current);
    let f = g + h;

    // Show current position
    let is_start = current == labyrinth.start;
    labyrinth.grid[current.0][current.1] = Cell::Visited;
    
    labyrinth.display();
    println!("Current: ({}, {})", current.0, current.1);

    //thread::sleep(Duration::from_millis(20));
    //std::io::stdout().flush();

    // Mark visited before returning early
    if f > bound {
        if !is_start {
            labyrinth.grid[current.0][current.1] = Cell::Visited;
        }

        return SearchResult::NewBound(f);
    }

    // Path found -> mark solution
    if current == labyrinth.goal {
        for &(x, y) in state.path.iter() {
            labyrinth.grid[x][y] = Cell::Solution;
        }

        //labyrinth.display();

        return SearchResult::Found(state.path.clone());
    }

    // Mark current cells as visited in state tracking
    state.visited[current.0][current.1] = true;

    // Get all possible moves and sort by estimated cost
    let mut moves = Vec::new();

    for (dx, dy) in [(0, 1), (1, 0), (0, -1), (-1, 0)] {
        let nx = (current.0 as isize + dx) as usize;
        let ny = (current.1 as isize + dy) as usize;

        if nx < labyrinth.width && ny < labyrinth.height && 
            labyrinth.grid[nx][ny] != Cell::Wall &&
            !state.visited[nx][ny] {
                let move_h = labyrinth.manhattan_distance((nx, ny));
                let move_g = g + 1;
                let move_f = move_g + move_h;

                moves.push((move_f, (nx, ny)));
        }
    }

    // Sort moves by cost
    moves.sort_by_key(|&(cost, _)| cost);

    let mut min_bound = usize::MAX;
    for (_, (nx, ny)) in moves {
        if !state.path.contains(&(nx, ny)) {
            state.path.push((nx, ny));

            // Recursively continue
            match search(labyrinth, g + 1, bound, state) {
                SearchResult::Found(solution) => return SearchResult::Found(solution),
                SearchResult::NewBound(new_bound) => {
                    min_bound = min_bound.min(new_bound);
                }
            }

            state.path.pop();
        }
    }

    // Mark current as visited before returning
    if !is_start {
        labyrinth.grid[current.0][current.1] = Cell::Visited;
    }

    SearchResult::NewBound(min_bound)
}

pub fn ida_star(labyrinth: &mut Labyrinth) -> Option<Vec<(usize, usize)>> {
    let initial_estimate = labyrinth.manhattan_distance(labyrinth.start);
    println!("MD = {}", initial_estimate);
    let mut bound = initial_estimate * 4;  // Start with a more generous bound
    let mut state = SearchState {
        path: vec![labyrinth.start],
        visited: vec![vec![false; labyrinth.width]; labyrinth.height],
    };

    while bound <= labyrinth.size * labyrinth.size {  // Upper limit to prevent infinite loops
        match search(labyrinth, 0, bound, &mut state) {
            SearchResult::Found(solution) => return Some(solution),
            SearchResult::NewBound(new_bound) => {
                if new_bound == usize::MAX {
                    bound += labyrinth.size;  // More aggressive bound increase
                } else {
                    bound = new_bound + labyrinth.size/2;  // Significant increase to reduce iterations
                }
                state.path = vec![labyrinth.start];  // Keep visited cells marked
            }
        }
    }
    None
}

pub fn welcome() {
    let default = String::from("an");
    println!("\n\n\tPentti got stuck in a maze.\n\tFinding a way out with {} algorithm", default);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn failing() {
        todo!();
    }
}
