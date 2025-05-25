use std::env;
use std::process;
use std::error::Error;
use std::io::Write;

use std::{thread, time::Duration};

use minotauros::{
    Config,
    Labyrinth,
};

fn run(args: Config) -> Result<(), Box<dyn Error>> {
    minotauros::welcome();

    let mut labyrinth = Labyrinth::new(&args);

    Labyrinth::display(&labyrinth); // Initial

    // Clear screen and hide cursor
    std::io::stdout().flush().unwrap();

    /* Basic WIP
    let start_point = &labyrinth.start.clone();
    basic_search(&mut labyrinth, start_point);
    */

    // A* WIP
    if let Some(solution) = minotauros::ida_star(&mut labyrinth) {
        println!("\nSolution found in {} steps!", solution.len());
        thread::sleep(Duration::from_secs(2));
    } else {
        println!("\nNo solution found...");
    }

    std::io::stdout().flush().unwrap();

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
