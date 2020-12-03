use std::collections::HashMap;
use std::collections::HashSet;
use std::cmp;

#[derive(Hash, Clone, Debug, Copy)]
struct Coordinate {
	x: i64,
	y: i64
}

impl Coordinate {
	fn new(x: i64, y: i64) -> Coordinate {
		Coordinate{x, y}
	}
}

impl Eq for Coordinate {}

impl PartialEq for Coordinate {
	fn eq(&self, other: &Self) -> bool {
		self.x == other.x && self.y == other.y
	}
}

fn silver(input: Vec<TYPE>) {

}

fn gold(input: Vec<TYPE>) {

}

fn main() {
	let input: Vec<TYPE> = include_str!("input")
		.trim()
		.split('\n')
		.collect();
}
