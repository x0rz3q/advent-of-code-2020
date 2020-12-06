use std::{collections::HashSet, time::Instant};

fn silver(input: &Vec<String>) -> usize {
	input
		.iter()
		.map(|x| x.replace("\n", "").chars().collect::<HashSet<char>>().len())
		.sum()
}

fn gold(input: &Vec<String>) -> usize {
	input
		.iter()
		.map(|x| {
			let len = x.matches("\n").count() + 1;

			x.replace("\n", "")
				.chars()
				.collect::<HashSet<char>>()
				.iter()
				.filter(|c| x.matches(&c.to_string()).count() == len)
				.count()
		})
		.sum()
}

fn main() {
	let now = Instant::now();

	let input: Vec<String> = include_str!("input")
		.trim()
		.split("\n\n")
		.map(|x| x.to_string())
		.collect();

	println!("Silver: {}", silver(&input));
	println!("Gold: {}", gold(&input));

	println!("Time: {}ms", now.elapsed().as_millis());
}
