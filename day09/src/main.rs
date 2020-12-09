use itertools::Itertools;
use std::time::Instant;

fn silver(input: &Vec<u64>, preamble: usize) -> (u64, usize) {
    for i in preamble..input.len() {
        let found = input[i - preamble..i]
            .iter()
            .cartesian_product(input[i - preamble..i].iter())
            .any(|(x, y)| x + y == input[i] && x != y);

        if !found {
            return (input[i], i);
        }
    }

    unreachable!();
}

fn gold(input: &Vec<u64>, target: u64) -> u64 {
    let mut left = 0;
    let mut right = 2;

    loop {
        let sum: u64 = input[left..right].iter().sum();
        if sum == target {
            let max = input[left..right].iter().max().unwrap();
            let min = input[left..right].iter().min().unwrap();

            return max + min;
        }

        if sum > target {
            left = left + 1;
        } else {
            right = right + 1;
        }
    }
}

fn main() {
    let now = Instant::now();

    let input: Vec<u64> = include_str!("input")
        .trim()
        .split('\n')
        .map(|x| x.parse().unwrap())
        .collect();

    let (result, index) = silver(&input, 25);
    println!("Silver: {}", result);

    println!("Gold: {}", gold(&input[..index].to_vec(), result));

    println!("Time: {}ms", now.elapsed().as_millis());
}
