use colored::*;
use itertools::Itertools;
use regex::Regex;
use std::{
	cmp,
	collections::{HashMap, HashSet, LinkedList, VecDeque},
	time::Instant,
};

use std::iter::FromIterator;

#[derive(Hash, Clone, Debug, Copy)]
struct Coordinate {
	x: i64,
	y: i64,
}

impl Coordinate {
	fn new(x: i64, y: i64) -> Coordinate {
		Coordinate { x, y }
	}
}

impl Eq for Coordinate {
}

impl PartialEq for Coordinate {
	fn eq(&self, other: &Self) -> bool {
		self.x == other.x && self.y == other.y
	}
}

fn silver(first_player: Vec<usize>, second_player: Vec<usize>) -> usize {
	let mut first_player = VecDeque::from_iter(first_player.iter());
	let mut second_player = VecDeque::from_iter(second_player.iter());

	let mut round = 1;

	while !first_player.is_empty() && !second_player.is_empty() {
		println!("-- Round {} --", round);
		println!("Player 1's deck: {:?}", first_player);
		println!("Player 2's deck: {:?}", second_player);

		let first = first_player.pop_front().unwrap();
		let second = second_player.pop_front().unwrap();

		println!("Player 1 plays: {}", first);
		println!("Player 2 plays: {}", second);

		if first > second {
			first_player.push_back(first);
			first_player.push_back(second);

			println!("Player 1 wins the round");
		} else {
			second_player.push_back(second);
			second_player.push_back(first);

			println!("Player 2 wins the round");
		}

		println!("");
		round += 1;
	}

	println!("== Post-game results ==");
	println!("Player 1's deck: {:?}", first_player);
	println!("Player 2's deck: {:?}", second_player);

	let mut sum = 0;
	let mut deck = first_player.clone();

	if first_player.is_empty() {
		deck = second_player.clone();
	}

	for m in 1..=deck.len() {
		sum += deck.pop_back().unwrap() * m;
	}

	sum
}

fn state_to_string(first_player: VecDeque<usize>, second_player: VecDeque<usize>) -> String {
	let mut state = first_player
		.iter()
		.map(|x| format!("{}", x).to_string())
		.join(" ");
	state.push_str(&"#");
	state.push_str(
		&second_player
			.iter()
			.map(|x| format!("{}", x).to_string())
			.join(" "),
	);

	state
}

fn run(
	game: usize,
	first_player: Vec<usize>,
	second_player: Vec<usize>,
) -> (VecDeque<usize>, VecDeque<usize>) {
	let mut first_player: VecDeque<usize> = VecDeque::from_iter(first_player.iter().map(|x| *x));
	let mut second_player: VecDeque<usize> = VecDeque::from_iter(second_player.iter().map(|x| *x));

	let mut seen = HashSet::new();

	while !first_player.is_empty() && !second_player.is_empty() {
		let state = state_to_string(first_player.clone(), second_player.clone());

		if seen.contains(&state.clone()) {
			return (first_player, VecDeque::new());
		}

		seen.insert(state.clone());

		let first = first_player.pop_front().unwrap();
		let second = second_player.pop_front().unwrap();

		if first <= first_player.len() && second <= second_player.len() {
			let winner = run(
				game + 1,
				Vec::from_iter(first_player.clone().iter().map(|x| *x).take(first)),
				Vec::from_iter(second_player.clone().iter().map(|x| *x).take(second)),
			);

			if !winner.0.is_empty() {
				first_player.push_back(first);
				first_player.push_back(second);
			} else {
				second_player.push_back(second);
				second_player.push_back(first);
			}
		} else {
			if first > second {
				first_player.push_back(first);
				first_player.push_back(second);
			} else {
				second_player.push_back(second);
				second_player.push_back(first);
			}
		}
	}

	(first_player, second_player)
}

fn gold(first_player: Vec<usize>, second_player: Vec<usize>) -> usize {
	let winner = run(1, first_player, second_player);

	let mut deck = winner.0;

	if deck.is_empty() {
		deck = winner.1;
	}

	let mut sum = 0;

	for m in 1..=deck.len() {
		sum += deck.pop_back().unwrap() * m;
	}

	sum
}

fn main() {
	let now = Instant::now();

	let input: Vec<Vec<usize>> = include_str!("input")
		.trim()
		.split("\n\n")
		.map(|x| x.split("\n").skip(1).map(|x| x.parse().unwrap()).collect())
		.collect();

	println!("Silver: {}", silver(input[0].clone(), input[1].clone()));
	println!("Gold: {}", gold(input[0].clone(), input[1].clone()));

	println!("Time: {}ms", now.elapsed().as_millis());
}
