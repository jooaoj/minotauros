use std::env;
use std::process;
use std::error::Error;

use minotauros::{
    Config,
    Labyrinth,
};

/*
// .lines()
// .map(|line| line.chars().collect())
// .collect();
fn scan_maze(maze_matrix: Vec<Vec<char>>) -> Vec<(usize, usize)> {
    /*
    let mut _ret_cur: [usize; 2] = [0, 0];
    let mut _dst: [usize; 2] = [0, 0];
    let mut _src_found: bool = false;
    let mut _dst_found: bool = true;
    */
    let mut ret_pos: Vec<(usize, usize)> = Vec::new();
    let mut ret_goal: Vec<(usize, usize)> = Vec::new();
    let ret: Vec<(usize, usize)> = Vec::new();

    for (y, row) in maze_matrix.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            match &col {
                '^' => ret_pos.push((x, y)),
                'E' => ret_goal.push((x, y)),
                _ => println!("{:?}", &row),
            }
            /*
            if !src_found {

                if let  //Some(src) = row.iter().position(|&caret| caret == '^') {
                    ret_cur[0] = Some(src).unwrap();
                    ret_cur[1] = y;
                    src_found = !src_found;
                }
            } 
            */
        }
    }
}

pub fn maze_routing_alg(maze: Vec<Vec<char>>) -> Result<(), Box<dyn Error>> {
    let points: Vec<(usize, usize)> = scan_maze(maze);
    let md_best = manhattan_distance(points[0], points[1]);

    println!("{:?}", points);

    Ok(())
}*/

struct SearchState {
    path: Vec<(usize, usize)>,
    visited: Vec<Vec<bool>>,
}

enum SearchResult {
    Found(Vec<(usize, usize)>),
    NewBound(usize),
}

/*
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
    labyrinth.grid[current.0][current.1] = Cell::Current;
    labyrinth.display();

    // Mark visited before returning early
    if f > bound {
        if !is_start {
            labyrinth.grid[current.0][current.1] = Cell::Visited;
        }

        return SearchResult(f);
    }

    // Path found -> mark solution
    if current == labyrinth.goal {
        for &(x, y) in state.path.iter() {
            labyrinth.grid[x][y] = Cell::Solution;
        }

        labyrinth.display();

        return SearchResult::Found(state.path.clone());
    }

    // Mark current cells as visited in state tracking
    state.visited[current.0][current.1] = true;

    // Get all possible moves and sort by estimated cost
    let mut moves = Vec::new();

    for (dx, dy) in [ (0, 1), (1, 0), (0, -1), (-1, 0) ] {
        let nx = (current.0 as isize + dx) as usize;
        let ny = (current.1 as isize + dy) as usize;

        if nx < labyrinth.size && ny < labyrinth.size && 
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
                SearchResult::Found(solution) => 
                    return SearchResult::Found(solution),
                SearchResult::NewBound(new_bound) => 
                    min_bound = min_bound.min(new_bound);
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
*/

fn run(args: Config) -> Result<(), Box<dyn Error>> {
    minotauros::welcome();

    let mut labyrinth = Labyrinth::new(&args);

    Labyrinth::display(&labyrinth);

    print!("{:?}", labyrinth.grid);

    println!("Start: {:?}, Goal: {:?}, Height: {:?}", labyrinth.start, labyrinth.goal, labyrinth.size);

    Ok(())
}

fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = run(config) {
        println!("Application error: {e}");
        process::exit(2);
    }
}
