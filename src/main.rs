use std::env;
use std::process;
use advent_of_code::Config;

fn main() {
    let config = Config::new(env::args());

    if let Err(e) = advent_of_code::run(config) {
    	eprintln!("Application error: {}", e);
    	process::exit(1);
    }
}
