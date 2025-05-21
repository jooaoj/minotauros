use std::error::Error;
use std::fs;

pub struct Args {
    pub file_path: String,
}

impl Args {
    pub fn build(args: &[String]) -> Result<Args, &'static str> {
       if args.len() == 1 {
            return Err("Not enough arguments: give a file path");
       }
       let file_path = args[1].clone();
       
       Ok(Args { file_path })
    }
}

pub fn run(args: Args) -> Result<(), Box<dyn Error>> {
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
