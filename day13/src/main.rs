use std::time::Instant;

// Credits go to: https://rosettacode.org/wiki/Chinese_remainder_theorem#Rust
fn egcd(a: i128, b: i128) -> (i128, i128, i128) {
    if a == 0 {
        (b, 0, 1)
    } else {
        let (g, x, y) = egcd(b % a, a);
        (g, y - (b / a) * x, x)
    }
}

// Credits go to: https://rosettacode.org/wiki/Chinese_remainder_theorem#Rust
fn mod_inv(x: i128, n: i128) -> Option<i128> {
    let (g, x, _) = egcd(x, n);
    if g == 1 {
        Some((x % n + n) % n)
    } else {
        None
    }
}

// Credits go to: https://rosettacode.org/wiki/Chinese_remainder_theorem#Rust
fn chinese_remainder(residues: &[i128], modulii: &[i128]) -> Option<i128> {
    let prod = modulii.iter().product::<i128>();

    let mut sum = 0;

    for (&residue, &modulus) in residues.iter().zip(modulii) {
        let p = prod / modulus;
        sum += residue * mod_inv(p, modulus)? * p
    }

    Some(sum % prod)
}

fn silver(timestamp: i128, lines: &Vec<i128>) -> i128 {
    let mut minimum = std::i128::MAX;
    let mut bus_id = 0;

    for bus in lines {
        if bus - (timestamp % bus) < minimum {
            bus_id = *bus;
            minimum = bus - (timestamp % bus)
        }
    }

    minimum * bus_id
}

fn gold(positions: &Vec<&str>) -> i128 {
    let mut modulii = Vec::new();
    let mut residues = Vec::new();

    for i in 0..positions.len() {
        if positions[i] != "x" {
            let bus = positions[i].parse().unwrap();
            residues.push(bus - (i as i128 % bus));
            modulii.push(bus);
        }
    }

    let result = chinese_remainder(&residues, &modulii);
    if result.is_none() {
        unreachable!();
    }

    result.unwrap()
}

fn main() {
    let now = Instant::now();

    let input: Vec<&str> = include_str!("input").trim().split('\n').collect();

    let timestamp: i128 = input[0].trim().parse().unwrap();
    let lines: Vec<i128> = input[1]
        .split(",")
        .filter(|x| *x != "x")
        .map(|x| x.parse().unwrap())
        .collect();

    println!("Silver: {}", silver(timestamp, &lines));
    println!("Gold: {}", gold(&input[1].split(",").collect()));

    println!("Time: {}ms", now.elapsed().as_millis());
}
