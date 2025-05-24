

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
