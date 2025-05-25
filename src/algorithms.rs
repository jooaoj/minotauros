/*
Public algorithm module-collection–modular and expandable.
–Jooa Jaakkola
*/

use minotauros::Labyrinth;
use minotauros::Cell;

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

pub fn a_star(labyrinth: &mut Labyrinth) -> Option<Vec<(usize, usize)>> {
    let initial_estimate = labyrinth.manhattan_distance(labyrinth.start);
    let mut bound = initial_estimate * 3;  // Start with a more generous bound
    let mut state = SearchState {
        path: vec![labyrinth.start],
        visited: vec![vec![false; labyrinth.size]; maze.size],
    };

    while bound < labyrinth.size * labyrinth.size {  // Upper limit to prevent infinite loops
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

fn basic_search(labyrinth: &mut Labyrinth, start_p: &(usize, usize)) -> bool {    
    let mut stack = VecDeque::new();
    stack.push_back(*start_p);

    while let Some(point) = stack.pop_back() {
        println!("Visiting {:?}", labyrinth.grid[point.0][point.1]);
        
        match labyrinth.grid[point.0][point.1] {
            Cell::Wall | Cell::Visited | Cell::Current => continue,
            Cell::Solution => return true,
            Cell::Path => labyrinth.grid[point.0][point.1] = Cell::Visited,
        }

        let rows = labyrinth.grid.len();
        let cols = labyrinth.grid[0].len();

        if point.0 + 1 < rows {
            stack.push_back((point.0 + 1, point.1));
        }
        if point.1 > 0 {
            stack.push_back((point.0, point.1 - 1));
        }
        if point.0 > 0 {
            stack.push_back((point.0 - 1, point.1));
        }
        if point.1 + 1 < cols {
            stack.push_back((point.0, point.1 + 1));
        }
    }

    false
}
