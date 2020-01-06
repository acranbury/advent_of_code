use std::fs;
use std::error::Error;

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
		"1" => day1(&contents),
		_ => day1(&contents),
	}

	Ok(())
}

fn day1(contents: &str)
{
	let mut fuel: i32 = 0;

	for line in contents.lines() {
		let mass: i32 = line.parse().unwrap();
		fuel += (mass / 3) - 2;
	}

	println!("Total fuel: {}", fuel);
}
