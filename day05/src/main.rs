use std::time::Instant;

fn process(current: (u64, u64), c: char, left: char, right: char) -> (u64, u64) {
	let bound = (current.0 + current.1) / 2;

	if c == left {
		return (current.0, bound);
	} else if c == right {
		return (bound + 1, current.1);
	}

	current
}

fn silver(input: Vec<Vec<char>>) -> u64 {
	input
		.iter()
		.map(|entry| {
			let row = entry
				.iter()
				.fold((0u64, 127u64), |current, c| process(current, *c, 'F', 'B'))
				.0;
			let column = entry
				.iter()
				.fold((0u64, 7u64), |current, c| process(current, *c, 'L', 'R'))
				.0;

			row * 8 + column
		})
		.max()
		.unwrap()
}

fn gold(input: Vec<Vec<char>>) -> u64 {
	let seats: Vec<u64> = input
		.iter()
		.map(|entry| {
			let row = entry
				.iter()
				.fold((0u64, 127u64), |current, c| process(current, *c, 'F', 'B'))
				.0;
			let column = entry
				.iter()
				.fold((0u64, 7u64), |current, c| process(current, *c, 'L', 'R'))
				.0;

			row * 8 + column
		})
		.collect();

	let minimum = *seats.iter().min().unwrap();
	let maximum = *seats.iter().max().unwrap();

	for seat in minimum..=maximum {
		if !seats.contains(&(seat as u64)) {
			return seat;
		}
	}

	unreachable!()
}

fn main() {
	let now = Instant::now();

	let input: Vec<Vec<char>> = include_str!("input")
		.trim()
		.split('\n')
		.map(|x| x.chars().collect())
		.collect();

	println!("Silver: {}", silver(input.clone()));
	println!("Gold: {}", gold(input));

	println!("Time: {}ms", now.elapsed().as_millis());
}
