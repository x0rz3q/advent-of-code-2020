use std::collections::HashMap;
use std::collections::HashSet;
use itertools::Itertools;
use std::time::Instant;
use regex::Regex;
use colored::*;
use std::cmp;


fn parse_list(input: &Vec<&str>) -> HashMap<String, Vec<String>> {
	let mut valid: HashMap<String, Vec<String>> = HashMap::new();

	for entry in input {
		let ingredients = entry.splitn(2, " (").nth(0).unwrap().split(" ");
		let allergies: Vec<&str> = entry.splitn(2, "(contains ").nth(1).unwrap().split(")").nth(0).unwrap().split(", ").collect();

		for allergy in allergies {
			if valid.contains_key(allergy) {
				let first: Vec<String> = valid.get(&allergy.to_string()).unwrap().clone();
				let second: Vec<String> = ingredients.clone().map(|x| x.to_string()).collect();

				let mut entry = HashSet::new();

				for i in first.clone() {
					if second.contains(&&i) {
						entry.insert(i);
					} 
				}

				valid.insert(allergy.to_string(), entry.iter().map(|x| x.to_string()).collect::<Vec<String>>());
			} else {
				valid.insert(allergy.to_string(), ingredients.clone().map(|x| x.to_string()).collect::<Vec<String>>());
			}
		}
	}

	valid
}

fn silver(input: &Vec<&str>) -> usize {
	let count = parse_list(input);
	let mut valid = HashSet::new();
	let mut sum = 0;

	for (_, value) in count {
		for i in value {
			valid.insert(i.to_string());
		}
	}

	for entry in input {
		let ingredients: Vec<&str> = entry.splitn(2, " (").nth(0).unwrap().split(" ").collect();

		sum += ingredients.iter().filter(|x| !valid.contains(&x.to_string())).count();
	}

	sum
}

fn recurse(valid: HashMap<String, Vec<String>>, allergies: &Vec<String>, index: usize, configuration: HashMap<String, String>) -> Option<HashMap<String, String>> {
	if valid.len() == configuration.clone().len() {
		return Some(configuration);
	}

	for value in valid.get(&allergies[index]).unwrap() {
		if !configuration.clone().contains_key(&value.to_string()) {
			let mut new = configuration.clone();
			new.insert(value.to_string(), allergies[index].clone());

			let result = recurse(valid.clone(), allergies, index + 1, new);
			if result.is_some() {
				return result;
			}
		}
	}

	None
}

fn gold(input: &Vec<&str>) -> String {
	let valid = parse_list(input);

	let allergies: Vec<String> = valid.iter().map(|(key, _)| key.to_string()).collect();

	let result = recurse(valid, &allergies, 0, HashMap::new()).unwrap();
	let mut result = result.iter().map(|(key, value)| (key.to_string(), value.to_string())).collect::<Vec<(String, String)>>();

	result.sort_by(|a, b| a.1.cmp(&b.1));

	result.iter().map(|x| x.0.clone()).join(",")
}

fn main() {
	let now = Instant::now();

	let input: Vec<&str> = include_str!("input")
		.trim()
		.split('\n')
		.collect();

	println!("Silver: {}", silver(&input));
	println!("Gold: {}", gold(&input));

	println!("Time: {}ms", now.elapsed().as_millis());
}
