use std::error::Error;

pub fn simple_r(maze: Vec<Vec<char>>) -> Result<(), Box<dyn Error>> {
	//let mut src: Option<usize>;
	let mut cur: Vec<usize> = Vec::new();
	let mut dst: Vec<Option<usize>> = Vec::new();

	let mut found: bool = false;

	// Scan maze
	for (y, row) in maze.iter().enumerate() {
		println!("{:?}", &row);
		for (x, col) in row.iter().enumerate() {
			if !found {
				if let Some(src) = row.iter().position(|&caret| caret == '^') {
					cur.push(Some(src).unwrap());
					cur.push(y);
					found = true;
				}
			} 
			//print!("{:?}", &col);
		}
		//println!("\n");
	}

	println!("{:?}", cur);

	Ok(())
}
