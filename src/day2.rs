pub fn run(contents: &str){
	let mut intcode: Vec<i32> = contents.split(',').map(|x| x.parse().unwrap()).collect();

	intcode_computer(&mut intcode);

	for code in intcode {
		println!("{},", code);
	}
}

fn computer(program: &Vec<i32>) {

}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn one_plus_one() {
		let mut actual = vec![1,0,0,0,99];
		let expected = vec![2,0,0,0,99];

		intcode_computer(&mut actual);

		assert_eq!(actual, expected);
	}

	#[test]
	fn three_times_two() {
		let mut actual = vec![2,3,0,3,99];
		let expected = vec![2,3,0,6,99];

		intcode_computer(&mut actual);

		assert_eq!(actual, expected);
	}

	#[test]
	fn ninety_nine_squared() {
		let mut actual = vec![2,4,4,5,99,0];
		let expected = vec![2,4,4,5,99,9801];

		intcode_computer(&mut actual);

		assert_eq!(actual, expected);
	}

	#[test]
	fn modify_code_before_executing() {
		let mut actual = vec![1,1,1,4,99,5,6,0,99];
		let expected = vec![30,1,1,4,2,5,6,0,99];

		intcode_computer(&mut actual);

		assert_eq!(actual, expected);
	}
}
