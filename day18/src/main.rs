use std::time::Instant;

fn recurse(parts: &Vec<String>, pointer: &mut usize) -> usize {
    let mut current = 0;
    let mut operation = "";

    while *pointer < parts.len() {
        let part = &parts[*pointer];
        *pointer += 1;

        if part == "(" {
            let result = recurse(parts, pointer);

            current = match operation {
                "" => result,
                "+" => current + result,
                "*" => current * result,
                _ => unreachable!(),
            };
        } else if part == "*" {
            operation = "*";
        } else if part == "+" {
            operation = "+";
        } else if part == ")" {
            return current;
        } else {
            let result = part.parse().unwrap();

            current = match operation {
                "" => result,
                "+" => current + result,
                "*" => current * result,
                _ => unreachable!(),
            };
        }
    }

    current
}

fn evaluate_simple(parts: &Vec<String>) -> usize {
    // remove + signs
    let mut chunks = Vec::new();
    let mut chunk = Vec::new();

    for i in 0..parts.len() {
        if parts[i] == "*" {
            chunks.push(chunk);
            chunk = Vec::new();
        } else if parts[i] != "+" {
            chunk.push(parts[i].clone());
        }
    }

    chunks.push(chunk);

    let mut product = 1;
    for chunk in chunks {
        let sum: usize = chunk.iter().map(|x| x.parse::<usize>().unwrap()).sum();
        product *= sum;
    }

    product
}

fn recurse_second(parts: &Vec<String>, pointer: &mut usize) -> usize {
    let mut parsed: Vec<String> = Vec::new();
    let mut operation = "";

    while *pointer < parts.len() {
        let part = &parts[*pointer];
        *pointer += 1;

        if part == "(" {
            let result = recurse_second(parts, pointer);

            parsed.push(result.to_string());
        } else if part == "*" || part == "+" {
            parsed.push(part.clone());
        } else if part == ")" {
            return evaluate_simple(&parsed);
        } else {
            parsed.push(part.clone());
        }
    }

    return evaluate_simple(&parsed);
}

fn get_chunks(input: &str) -> Vec<String> {
    input
        .chars()
        .filter(|c| *c != ' ')
        .map(|x| x.to_string())
        .collect()
}

fn silver(input: &Vec<&str>) -> usize {
    let mut sum = 0;

    for entry in input {
        let parts = get_chunks(entry);

        sum += recurse(&parts, &mut 0);
    }

    sum
}

fn gold(input: Vec<&str>) -> usize {
    let mut sum = 0;

    for entry in input {
        let parts = get_chunks(entry);

        sum += recurse_second(&parts, &mut 0);
    }

    sum
}

fn main() {
    let now = Instant::now();

    let input: Vec<&str> = include_str!("input").trim().split('\n').collect();

    println!("Silver: {}", silver(&input));
    println!("Gold: {}", gold(input));

    println!("Time: {}ms", now.elapsed().as_millis());
}

#[test]
fn silver_first() {
    let parts = get_chunks("1 + 2 * 3 + 4 * 5 + 6");

    assert_eq!(71, recurse(&parts, &mut 0))
}

#[test]
fn silver_second() {
    let parts = get_chunks("1 + (2 * 3) + (4 * (5 + 6))");

    assert_eq!(51, recurse(&parts, &mut 0))
}

#[test]
fn silver_third() {
    let parts = get_chunks("2 * 3 + (4 * 5)");

    assert_eq!(26, recurse(&parts, &mut 0))
}

#[test]
fn silver_fourth() {
    let parts = get_chunks("5 + (8 * 3 + 9 + 3 * 4 * 3)");

    assert_eq!(437, recurse(&parts, &mut 0))
}

#[test]
fn silver_fith() {
    let parts = get_chunks("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))");

    assert_eq!(12240, recurse(&parts, &mut 0))
}

#[test]
fn silver_sixth() {
    let parts = get_chunks("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2");

    assert_eq!(13632, recurse(&parts, &mut 0))
}

#[test]
fn test_simple() {
    let parts = get_chunks("1 + 2 * 3 + 4 * 5 + 6");

    assert_eq!(231, evaluate_simple(&parts));
}

#[test]
fn gold_first() {
    let parts = get_chunks("1 + (2 * 3) + (4 * (5 + 6))");

    assert_eq!(51, recurse_second(&parts, &mut 0))
}

#[test]
fn gold_second() {
    let parts = get_chunks("2 * 3 + (4 * 5)");

    assert_eq!(46, recurse_second(&parts, &mut 0))
}

#[test]
fn gold_third() {
    let parts = get_chunks("5 + (8 * 3 + 9 + 3 * 4 * 3)");

    assert_eq!(1445, recurse_second(&parts, &mut 0))
}

#[test]
fn gold_fourth() {
    let parts = get_chunks("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))");

    assert_eq!(669060, recurse_second(&parts, &mut 0))
}

#[test]
fn gold_fifth() {
    let parts = get_chunks("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2");

    assert_eq!(23340, recurse_second(&parts, &mut 0))
}
