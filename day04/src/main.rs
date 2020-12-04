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
    let eyes = vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
    let hcl_re = Regex::new(r"#[0-9a-f]{6}").unwrap();

    let input: Vec<String> = input
        .iter()
        .filter(|x| constraints.iter().all(|y| x.matches(y).count() == 1))
        .map(|x| x.to_string())
        .collect();

    let mut valid = 0;
    for entry in input {
        let map = entry
            .split_whitespace()
            .flat_map(|p| p.split(':'))
            .tuples()
            .collect::<HashMap<_, _>>();

        let byr = map.get("byr").unwrap().parse::<u64>();
        let iyr = map.get("iyr").unwrap().parse::<u64>();
        let eyr = map.get("eyr").unwrap().parse::<u64>();

        if byr.is_err() || iyr.is_err() || eyr.is_err() {
            println!("{}", "Invalid byr/iyr/eyr".red());
            continue;
        }

        let byr = byr.unwrap();
        let iyr = iyr.unwrap();
        let eyr = eyr.unwrap();

        if !((1920 <= byr && byr <= 2002)
            && (2010 <= iyr && iyr <= 2020)
            && (2020 <= eyr && eyr <= 2030))
        {
            println!("{}", "Invalid byr/iyr/eyr value".red());
            continue;
        }

        let hgt = map.get("hgt").unwrap();
        if hgt.matches("in").count() == 0 && hgt.matches("cm").count() == 0 {
            println!("{}", "Invalid hgt format".red());
            continue;
        }

        let is_cm = hgt.matches("cm").count() == 1;
        let hgt = hgt.replace("in", "").replace("cm", "");
        let hgt = hgt.parse::<u64>();

        if hgt.is_err() {
            println!("{}", "Invalid hgt integer".red());
            continue;
        }

        let hgt = hgt.unwrap();
        if !((is_cm && 150 <= hgt && hgt <= 193) || (!is_cm && 59 <= hgt && hgt <= 76)) {
            println!(
                "{}",
                format!("Invalid hgt value {} with is_cm {}", hgt, is_cm).red()
            );
            continue;
        }

        let hcl = map.get("hcl").unwrap();
        if !hcl_re.is_match(hcl) {
            println!("{}", format!("Invalid hcl value {}", hcl).red());
            continue;
        }

        let pid = map.get("pid").unwrap();
        if pid.len() != 9 || pid.parse::<u64>().is_err() {
            println!("{}", format!("Invalid pid {}", pid).red());
            continue;
        }

        let ecl = map.get("ecl").unwrap();
        if !eyes.iter().any(|x| x == ecl) {
            println!("{}", format!("Invalid ecl {}", ecl).red());
            continue;
        }

        valid += 1;
    }

    valid
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
