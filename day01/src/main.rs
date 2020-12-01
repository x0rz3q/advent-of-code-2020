#[macro_use] 
extern crate itertools;

fn silver(input: Vec<i64>) -> i64 {
	for (x, y) in iproduct!(input.clone(), input) {
		if x + y == 2020 {
			return x * y;
		}
	}

	0
}

fn gold(input: Vec<i64>) -> i64 {
	for (x, y, z) in iproduct!(input.clone(), input.clone(), input) {
		if x + y + z == 2020 {
			return x * y * z;
		}
	}
	
	0
}

fn main() {
	let input: Vec<i64> = include_str!("input")
		.trim()
		.split('\n')
		.map(|num| num.parse::<i64>().unwrap())
		.collect();

	println!("Silver: {}", silver(input.clone()));
	println!("Gold: {}", gold(input));
}
