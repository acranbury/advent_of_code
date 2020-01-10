pub fn run(contents: &str) {
	let mut fuel: i32 = 0;

	for line in contents.lines() {
		let mass: i32 = line.parse().unwrap();
		fuel += get_fuel_cost(mass);
	}

	println!("Total fuel: {}", fuel);
}

fn get_fuel_cost(mass: i32) -> i32 {
	let fuel = fuel_calc(mass);
	if fuel <= 0 {
		0
	} else {
		fuel + get_fuel_cost(fuel)
	}
}

fn fuel_calc(mass: i32) -> i32 {
	(mass / 3) - 2
}

#[cfg(test)]
mod day1_tests {
	use super::*;

	#[test]
	fn simple_fuel_calcs() {
		assert_eq!(fuel_calc(1969), 654);
		assert_eq!(fuel_calc(100756), 33583);
	}

	#[test]
	fn complex_fuel_calcs() {
		assert_eq!(get_fuel_cost(1969), 966);
		assert_eq!(get_fuel_cost(100756), 50346);
	}
}