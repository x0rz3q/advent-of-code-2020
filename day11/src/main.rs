use colored::*;
use std::thread;
use std::time;
use std::time::Instant;

fn count_adjacent(input: &Vec<Vec<char>>, (x, y): (usize, usize)) -> usize {
    let mut count = 0;
    let adjacent: Vec<(i32, i32)> = vec![
        (1, 0),
        (0, 1),
        (1, 1),
        (-1, 0),
        (0, -1),
        (-1, -1),
        (1, -1),
        (-1, 1),
    ];

    for a in adjacent {
        let (left, top) = a;
        let left = left + x as i32;
        let top = top + y as i32;

        if left >= 0 && top >= 0 && left < input.len() as i32 && top < input[x].len() as i32 {
            count += match input[left as usize][top as usize] {
                'L' => 0,
                '.' => 0,
                '#' => 1,
                _ => unreachable!(),
            };
        }
    }

    count
}

fn count_visible(input: &Vec<Vec<char>>, (x, y): (usize, usize)) -> usize {
    let mut count = 0;
    let adjacent: Vec<(i32, i32)> = vec![
        (1, 0),
        (0, 1),
        (1, 1),
        (-1, 0),
        (0, -1),
        (-1, -1),
        (1, -1),
        (-1, 1),
    ];

    for a in adjacent {
        let (left, top) = a;
        let mut left = left + x as i32;
        let mut top = top + y as i32;

        loop {
            if left >= 0 && top >= 0 && left < input.len() as i32 && top < input[x].len() as i32 {
                count += match input[left as usize][top as usize] {
                    'L' => 0,
                    '.' => 0,
                    '#' => 1,
                    _ => unreachable!(),
                };

                if input[left as usize][top as usize] == '.' {
                    left = left + a.0 as i32;
                    top = top + a.1 as i32;
                } else {
                    break;
                }
            } else {
                break;
            }
        }
    }

    count
}

fn step(input: &mut Vec<Vec<char>>, limit: usize, visible: bool) -> usize {
    let mut switch: Vec<(usize, usize, char)> = Vec::new();
    let mut changes = 0;

    for x in 0..input.len() {
        for y in 0..input[x].len() {
            let mut adjacent = 0;
            if !visible {
                adjacent = count_adjacent(&input, (x, y));
            } else {
                adjacent = count_visible(&input, (x, y));
            }

            if adjacent == 0 && input[x][y] == 'L' {
                changes += 1;
                switch.push((x, y, '#'));
            } else if adjacent >= limit && input[x][y] == '#' {
                changes += 1;
                switch.push((x, y, 'L'));
            }
        }
    }

    for (x, y, c) in switch {
        input[x][y] = c;
    }

    changes
}

fn print(input: &Vec<Vec<char>>) {
    thread::sleep(time::Duration::from_millis(300));

    let mut text: String = "".to_string();
    for x in 0..input.len() {
        for y in 0..input[x].len() {
            let s = match input[x][y] {
                '.' => format!("{}", input[x][y]).black(),
                '#' => format!("{}", input[x][y]).red(),
                'L' => format!("{}", input[x][y]).green(),
                _ => unreachable!(),
            };

            text.push_str(&s.to_string());
        }

        text.push('\n');
    }

    println!("{}", text);
}

fn silver(input: &mut Vec<Vec<char>>) -> usize {
    loop {
        let changes = step(input, 4, false);

        if changes == 0 {
            break;
        }
    }

    input
        .iter()
        .map(|x| x.iter().filter(|y| **y == '#').count())
        .sum()
}

fn gold(input: &mut Vec<Vec<char>>) -> usize {
    loop {
        let changes = step(input, 5, true);

        if changes == 0 {
            break;
        }
    }

    input
        .iter()
        .map(|x| x.iter().filter(|y| **y == '#').count())
        .sum()
}

fn main() {
    let now = Instant::now();

    let mut input: Vec<Vec<char>> = include_str!("input")
        .trim()
        .split('\n')
        .map(|x| x.chars().collect())
        .collect();

    println!("Silver: {}", silver(&mut input.clone()));
    println!("Gold: {}", gold(&mut input.clone()));

    println!("Time: {}ms", now.elapsed().as_millis());
}
