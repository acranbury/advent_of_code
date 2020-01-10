use std::process;

pub fn run(contents: &str){
	let mut intcode: Vec<i32> = contents.split(',')
										.map(|x| x.parse().unwrap())
										.collect();

	// Restore to "1202 program alarm" state
	intcode[1] = 12;
	intcode[2] = 2;

	intcode_computer(&mut intcode);

	println!("{:?},", intcode);
}

const OPCODE_ADD: i32 = 1;
const OPCODE_MUL: i32 = 2;

pub fn intcode_computer(program: &mut Vec<i32>) {
	let mut pc: usize = 0;
	let mut opcode = program[pc];

	while opcode != 99 {
		match opcode {
			OPCODE_ADD => {
				let param1: usize = program[pc + 1] as usize;
				let param2: usize = program[pc + 2] as usize;
				let result: usize = program[pc + 3] as usize;
				program[result] = program[param1] + program[param2];
			},
			OPCODE_MUL => {
				let param1: usize = program[pc + 1] as usize;
				let param2: usize = program[pc + 2] as usize;
				let result: usize = program[pc + 3] as usize;
				program[result] = program[param1] * program[param2];
			},
			_ => {
				println!("Unknown Opcode! {}", opcode);
				process::exit(1);
			},
		};

		pc += 4;

		if pc < program.len() {
			opcode = program[pc];
		} else {
			println!("Program ended unexpectedly! pc: {}", pc);
			process::exit(1);
		}
	}
}

#[cfg(test)]
mod day2_tests {
	use super::*;

	#[test]
	fn one_plus_one() {
		let mut actual = vec![1,0,0,0,99];
		let expected   = vec![2,0,0,0,99];

		intcode_computer(&mut actual);

		assert_eq!(actual, expected);
	}

	#[test]
	fn three_times_two() {
		let mut actual = vec![2,3,0,3,99];
		let expected   = vec![2,3,0,6,99];

		intcode_computer(&mut actual);

		assert_eq!(actual, expected);
	}

	#[test]
	fn ninety_nine_squared() {
		let mut actual = vec![2,4,4,5,99,0];
		let expected   = vec![2,4,4,5,99,9801];

		intcode_computer(&mut actual);

		assert_eq!(actual, expected);
	}

	#[test]
	fn modify_code_before_executing() {
		let mut actual = vec![1,1,1,4,99,5,6,0,99];
		let expected   = vec![30,1,1,4,2,5,6,0,99];

		intcode_computer(&mut actual);

		assert_eq!(actual, expected);
	}
}
