extern crate regex;

use regex::Regex;

fn silver(input: Vec<String>) -> u64 {
	let mut count: u64 = 0;
	let expression = r"(?P<lower>\d+)-(?P<upper>\d+) (?P<character>\w+): (?P<candidate>\w+)";
	let re = Regex::new(expression).unwrap();

	for password in input {
		let capture = re
			.captures(&password)
			.expect("Regex capture resulted in a None");

		let lower: usize = capture
			.name("lower")
			.unwrap()
			.as_str()
			.parse::<usize>()
			.unwrap();
		let upper: usize = capture
			.name("upper")
			.unwrap()
			.as_str()
			.parse::<usize>()
			.unwrap();
		let character = capture.name("character").unwrap().as_str();
		let candidate: String = String::from(capture.name("candidate").unwrap().as_str());

		let c = candidate.matches(character).count();

		if c >= lower && c <= upper {
			count += 1;
		}
	}

	count
}

fn gold(input: Vec<String>) -> u64 {
	let mut count: u64 = 0;
	let expression = r"(?P<lower>\d+)-(?P<upper>\d+) (?P<character>\w+): (?P<candidate>\w+)";
	let re = Regex::new(expression).unwrap();

	for password in input {
		let capture = re
			.captures(&password)
			.expect("Regex capture resulted in a None");

		let lower: usize = capture
			.name("lower")
			.unwrap()
			.as_str()
			.parse::<usize>()
			.unwrap();
		let upper: usize = capture
			.name("upper")
			.unwrap()
			.as_str()
			.parse::<usize>()
			.unwrap();
		let character = capture.name("character").unwrap().as_str();
		let candidate: String = String::from(capture.name("candidate").unwrap().as_str());

		let first = candidate.chars().nth(lower - 1).unwrap().to_string();
		let second = candidate.chars().nth(upper - 1).unwrap().to_string();

		if (first == character) ^ (second == character) {
			count += 1;
		}
	}

	count
}

fn main() {
	let input: Vec<String> = include_str!("input")
		.trim()
		.split('\n')
		.map(|s| s.to_string())
		.collect();

	println!("Silver: {}", silver(input.clone()));
	println!("Gold: {}", gold(input));
}
