use std::fs;
use std::error::Error;

mod day1;
mod day2;

pub struct Config {
	pub day: String
}

impl Config {
	pub fn new(mut args: std::env::Args) -> Config {
		args.next();

		let day = match args.next() {
			Some(arg) => arg,
			None => String::from("1"),
		};

		Config { day }
	}
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
	let contents = fs::read_to_string(format!("input{}.txt", &config.day))?;

	match config.day.as_ref() {
		"1" => day1::run(&contents),
		"2" => day2::run(&contents),
		_ => day1::run(&contents),
	}

	Ok(())
}
