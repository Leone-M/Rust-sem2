use minigrep::{run, Config};
use std::env;
use std::process;

fn main() {
    // command line arguments
    let args: Vec<String> = env::args().collect();

    // getting arguments through lib logic
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Something gone wrong!\n{err}");
        process::exit(1);
    });

    if let Err(e) = run(config) {
        println!("Application error!\n{e}");
        process::exit(1);
    };
}
