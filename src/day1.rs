pub fn run(contents: &str) {
	let mut fuel: i32 = 0;

	for line in contents.lines() {
		let mass: i32 = line.parse().unwrap();
		fuel += get_fuel_cost(mass);
	}

	println!("Total fuel: {}", fuel);
}

pub fn get_fuel_cost(mass: i32) -> i32 {
	let fuel =(mass / 3) - 2;
	if fuel <= 0 {
		0
	} else {
		fuel + get_fuel_cost(fuel)
	}
}