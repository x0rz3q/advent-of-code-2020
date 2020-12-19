use colored::*;
use itertools::Itertools;
use regex::Regex;
use std::cmp;
use std::collections::HashMap;
use std::collections::HashSet;
use std::time::Instant;

fn build_rule(map: &HashMap<String, String>, rule: &str) -> Vec<String> {
    let mut entries = Vec::new();

    let requirements: Vec<&str> = map.get(&rule.to_string()).unwrap().split(" | ").collect();

    for requirement in requirements {
        let mut comparitor: Vec<String> = Vec::new();
        comparitor.push("".to_string());

        let parts: Vec<&str> = requirement.split(" ").collect();

        for part in parts {
            if part.chars().all(|c| c.is_ascii_digit()) {
                let mut new_comparitor: Vec<String> = Vec::new();

                let subrules = build_rule(map, part);

                for subrule in subrules {
                    for comp in &comparitor {
                        let mut entry = comp.clone();
                        entry.push_str(&subrule.to_string());

                        new_comparitor.push(entry);
                    }
                }

                comparitor = new_comparitor;
            } else {
                let inject = part.to_string().replace('\"', "");

                let mut new_comparitor: Vec<String> = Vec::new();

                for comp in comparitor {
                    let mut entry = comp.clone();
                    entry.push_str(&inject);

                    new_comparitor.push(entry);
                }

                comparitor = new_comparitor;
            }
        }

        entries.append(&mut comparitor);
    }

    entries
}

fn silver(rules: &HashMap<String, String>, entries: Vec<&str>) -> usize {
    let matchers = build_rule(rules, &"0");

    entries
        .iter()
        .filter(|x| matchers.contains(&x.to_string()))
        .count()
}

fn gold(rules: &HashMap<String, String>, entries: Vec<&str>) -> usize {
    // 0: 8 11

    let prefix = build_rule(rules, &"42");
    let suffix = build_rule(rules, &"31");

    let mut result = 0;
    for entry in entries {
        // the prefix and suffix all have parts of 8 characters.
        if entry.len() % 8 == 0 {
            let chunks = entry
                .chars()
                .collect::<Vec<char>>()
                .chunks(8)
                .map(|c| c.iter().collect::<String>())
                .collect::<Vec<String>>();

            let mut counter = 0;

            // find all prefixes e.g. 42 blocks
            while counter < chunks.len() {
                if prefix.contains(&chunks[counter]) {
                    counter += 1;
                } else {
                    break;
                }
            }

            let mut prefix_counter: i64 = counter as i64;

            // loop over all 31 blocks and decrement a 42 block pointer
            while counter < chunks.len() {
                if suffix.contains(&chunks[counter]) {
                    counter += 1;
                    prefix_counter -= 1;
                } else {
                    break;
                }
            }

            // if we counted all of them and we have at least one more prefix than suffix, and the
            // first part is a prefix and the last part is a suffix. Then we have found one
            if counter == chunks.len()
                && prefix_counter >= 1
                && prefix.contains(&chunks[0])
                && suffix.contains(chunks.last().unwrap())
            {
                result += 1;
            }
        }
    }

    result
}

fn main() {
    let now = Instant::now();

    let blocks: Vec<&str> = include_str!("input").trim().split("\n\n").collect();

    let mut rules: HashMap<String, String> = HashMap::new();

    for rule in blocks[0].split('\n') {
        let mut parts = rule.splitn(2, ": ");

        rules.insert(
            parts.next().unwrap().to_string(),
            parts.next().unwrap().to_string(),
        );
    }

    println!(
        "Silver: {}",
        silver(&rules, blocks[1].split("\n").collect::<Vec<&str>>())
    );
    println!(
        "Gold: {}",
        gold(&rules, blocks[1].split("\n").collect::<Vec<&str>>())
    );

    println!("Time: {}ms", now.elapsed().as_millis());
}
