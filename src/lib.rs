use std::error::Error;
use std::fs;

mod algos;

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

       algos::Graph { sink: String::from("") };
       
       Ok(Config { file_path })
    }
}

pub fn run(args: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(args.file_path)?;

    welcome();

    println!("\n\n\tLabyrinth:\n\n{contents}");

    Ok(())
}

fn welcome() {
    let default = String::from("Trémaux's");
    println!("\n\n\tPentti got stuck in a labyrinth.\n\tFinding a way out with: {}", default);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn failing() {
        assert_eq!(0, 1);
    }
}
