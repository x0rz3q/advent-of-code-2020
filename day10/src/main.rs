use std::collections::HashMap;
use std::collections::HashSet;

use std::time::Instant;

fn silver(input: &Vec<u64>) -> u64 {
    let mut input = input.clone();
    let mut differences = (0, 0);

    input.sort();
    for i in 1..input.len() {
        differences = match input[i] - input[i - 1] {
            1 => (differences.0 + 1, differences.1),
            3 => (differences.0, differences.1 + 1),
            _ => unreachable!(),
        }
    }

    differences.0 * differences.1
}

fn backtrack(end: u64, index: u64, cache: &mut HashMap<u64, u64>, input: &HashSet<u64>)  -> u64 {
    let result = if index == end {
        1
    } else if !input.contains(&index) {
        0
    } else if cache.contains_key(&index) {
        *cache.get(&index).unwrap()
    } else {
        (1..=3)
            .map(|x| backtrack(end, index + x, cache, input))
            .sum()
    };

    cache.insert(index, result);

    result
}

fn gold(input: &Vec<u64>) -> u64 {
    let mut cache = HashMap::new();
    let set: HashSet<u64> = input.iter().map(|x| *x).collect();

    backtrack(*input.last().unwrap(), 0, &mut cache, &set)
}

fn main() {
    let now = Instant::now();

    let mut input: Vec<u64> = include_str!("input")
        .trim()
        .split('\n')
        .map(|x| x.parse().unwrap())
        .collect();

    input.push(0);
    input.push(input.iter().max().unwrap() + 3);

    println!("Silver: {}", silver(&input));
    println!("Gold: {}", gold(&input));

    println!("Time: {}ms", now.elapsed().as_millis());
}
