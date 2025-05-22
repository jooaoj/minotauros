use std::error::Error;
use std::fs;

mod algorithms;

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

pub fn run(args: Config) -> Result<(), Box<dyn Error>> {
    let maze_source = fs::read_to_string(args.file_path)?;
    let maze_matrix: Vec<Vec<char>> = maze_source
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    //println!("\n\nMaze:\n\n{:?}", maze_source);

    welcome();

    algorithms::simple_r(maze_matrix);

    Ok(())
}

fn welcome() {
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
