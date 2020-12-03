use std::collections::HashMap;

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

fn walk(map: &HashMap<Coordinate, char>, height: i64, width: i64, step_x: i64, step_y: i64) -> i64 {
	let mut y = 0;
	let mut x = 0;
	let mut trees = 0;

	while y < height {
		let coordinate = Coordinate::new(x, y);
		let c = map.get(&coordinate).unwrap();

		if c == &'#' {
			trees += 1;
		}

		y = y + step_y;
		x = (x + step_x) % width;
	}

	trees
}

fn silver(map: &HashMap<Coordinate, char>, height: i64, width: i64) -> i64 {
	walk(map, height, width, 3, 1)
}

fn gold(map: &HashMap<Coordinate, char>, height: i64, width: i64, steps: Vec<(i64, i64)>) -> i64 {
	let mut trees = 1;

	for (x, y) in steps {
		trees = trees * walk(map, height, width, x, y);
	}

	trees
}

fn main() {
	let input: Vec<String> = include_str!("input")
		.trim()
		.split('\n')
		.map(|x| x.to_string())
		.collect();

	let mut map = HashMap::new();
	let mut y = 0;
	let mut width = 0;
	let height = input.len() as i64;

	for line in input.clone() {
		let mut x = 0;
		for c in line.chars() {
			map.insert(Coordinate::new(x, y), c);

			x += 1;

			if width < x {
				width = x;
			}
		}

		y += 1;
	}

	println!("Silver: {}", silver(&map, height, width));

	let steps = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
	println!("Gold: {}", gold(&map, height, width, steps));
}
