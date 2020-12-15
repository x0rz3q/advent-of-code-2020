use colored::*;
use itertools::Itertools;
use regex::Regex;
use std::cmp;
use std::collections::HashMap;
use std::collections::HashSet;
use std::time::Instant;

fn run(input: &Vec<usize>, limit: usize) -> usize {
    let mut memory: HashMap<usize, Vec<usize>> = HashMap::new();
    let mut turn = 1;
    let mut last = 0;

    for k in input {
        let mut history = Vec::new();
        history.push(turn);
        memory.insert(*k, history);

        turn += 1;
        last = *k;
    }

    while turn <= limit {
        let mut entry = memory.get(&last).unwrap();

        if entry.len() >= 2 {
            let first = entry[entry.len() - 2];
            let second = entry[entry.len() - 1];
            last = second - first;
        } else {
            last = 0;
        }

        let mut entry = memory.entry(last).or_insert(Vec::new());
        entry.push(turn);

        turn += 1;
    }

    last

}

fn silver(input: &Vec<usize>) -> usize {
	run(input, 2020)
}

fn gold(input: &Vec<usize>) -> usize {
	run(input, 30000000)
}

fn main() {
    let now = Instant::now();

    let input: Vec<usize> = include_str!("input")
        .trim()
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect();

    println!("Silver: {}", silver(&input));
    println!("Gold: {}", gold(&input));

    println!("Time: {}ms", now.elapsed().as_millis());
}
