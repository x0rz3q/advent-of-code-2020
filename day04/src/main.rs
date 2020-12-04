use colored::*;
use itertools::Itertools;
use regex::Regex;
use std::collections::HashMap;
use std::time::Instant;

fn silver(input: &Vec<String>) -> usize {
    let constraints = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

    input
        .iter()
        .filter(|x| constraints.iter().all(|y| x.matches(y).count() == 1))
        .count()
}

fn gold(input: &Vec<String>) -> usize {
    let constraints = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

    let input: Vec<String> = input
        .iter()
        .filter(|x| constraints.iter().all(|y| x.matches(y).count() == 1))
        .map(|x| x.to_string())
        .collect();

    let mut comparitor: HashMap<&str, Regex> = HashMap::new();
    comparitor.insert(
        "byr",
        Regex::new(r"^19[2-8][0-9]|199[0-9]|200[0-2]$").unwrap(),
    );
    comparitor.insert("iyr", Regex::new(r"^(201[0-9]|2020)$").unwrap());
    comparitor.insert("eyr", Regex::new(r"^(202[0-9]|2030)$").unwrap());
    comparitor.insert(
        "hgt",
        Regex::new(r"^((1[5-8][0-9]|19[0-3])cm|(59|6[0-9]|7[0-6])in)$").unwrap(),
    );
    comparitor.insert("hcl", Regex::new(r"^#[0-9a-f]{6}$").unwrap());
    comparitor.insert("ecl", Regex::new(r"^amb|blu|brn|gry|grn|hzl|oth$").unwrap());
    comparitor.insert("pid", Regex::new(r"^[0-9]{9}$").unwrap());
    comparitor.insert("cid", Regex::new(r"").unwrap());

    input
        .iter()
        .map(|entry| {
            entry
                .split_whitespace()
                .flat_map(|p| p.split(':'))
                .tuples()
                .all(|(key, value)| comparitor.get(key).unwrap().is_match(value))
        })
        .filter(|x| *x)
        .count()
}

fn main() {
    let now = Instant::now();

    let input: Vec<String> = include_str!("input")
        .trim()
        .split("\n\n")
        .map(|x| x.to_string())
        .collect();

    println!("{}", format!("Silver: {}", silver(&input)).green());
    println!("{}", format!("Gold: {}", gold(&input)).green());

    println!("Time: {}ms", now.elapsed().as_millis());
}
