use std::env;
use std::process;

use minotauros::Args;

fn main() {
    let _args: Vec<String> = env::args().collect();

    let args = Args::build(&_args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = minotauros::run(args) {
        println!("Application error: {e}");
        process::exit(2);
    }
}
