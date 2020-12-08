use std::collections::HashSet;
use std::time::Instant;

fn run(input: &Vec<String>) -> (bool, i32) {
    let mut acc = 0;
    let mut pointer = 0;
    let mut history = HashSet::new();

    while pointer < input.len() {
        if history.contains(&pointer) {
            return (false, acc);
        }

        history.insert(pointer);
        let instruction = input.get(pointer).unwrap();

        match &instruction[..3] {
            "acc" => {
                acc = acc + instruction[4..].parse::<i32>().unwrap();
                pointer = pointer + 1;
            }
            "jmp" => pointer = (pointer as i32 + instruction[4..].parse::<i32>().unwrap()) as usize,
            "nop" => pointer = pointer + 1,
            _ => unreachable!(),
        }
    }

    return (true, acc);
}

fn silver(input: &Vec<String>) -> i32 {
    run(input).1
}

fn gold(input: &Vec<String>) -> i32 {
    let mut input = input.clone();

    for i in 0..input.len() {
        let entry = input.get(i).unwrap().clone();

        if entry.contains("jmp") {
            let new = entry.replace("jmp", "nop");
            input[i] = new;

            let (result, acc) = run(&input);
            if result {
                return acc;
            }

            input[i] = entry;
        } else if entry.contains("nop") {
            let new = entry.replace("nop", "jmp");
            input[i] = new;

            let (result, acc) = run(&input);
            if result {
                return acc;
            }

            input[i] = entry;
        }
    }

    unreachable!()
}

fn main() {
    let now = Instant::now();

    let input: Vec<String> = include_str!("input")
        .trim()
        .split('\n')
        .map(|x| x.to_string())
        .collect();

    println!("Silver: {}", silver(&input));
    println!("Gold: {}", gold(&input));

    println!("Time: {}ms", now.elapsed().as_millis());
}
