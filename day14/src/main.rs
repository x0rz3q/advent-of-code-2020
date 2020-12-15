use colored::*;
use itertools::Itertools;
use regex::Regex;
use std::cmp;
use std::collections::HashMap;
use std::collections::HashSet;
use std::time::Instant;

struct Computer {
    memory: HashMap<usize, usize>,
}

#[derive(Clone)]
struct Mask {
    mask: Vec<char>,
    blocks: Vec<String>,
}

impl Mask {
    fn new(line: &str) -> Mask {
        Mask {
            mask: line
                .splitn(2, " = ")
                .nth(1)
                .unwrap()
                .chars()
                .collect::<Vec<char>>()
                .clone(),
            blocks: Vec::new(),
        }
    }

    fn get_or_mask(&self) -> usize {
        let mut mask = 0;

        for c in self.mask.iter() {
            mask = mask << 1;
            mask = mask | ((*c == '1') as usize);
        }

        mask
    }

    fn get_and_mask(&self) -> usize {
        let mut mask = 0;

        for c in self.mask.iter() {
            mask = mask << 1;
            mask = mask | ((*c != '0') as usize);
        }

        mask
    }

    fn get_value(&self, value: usize) -> usize {
        (value & self.get_and_mask()) | self.get_or_mask()
    }
}

impl Computer {
    fn new() -> Computer {
        Computer {
            memory: HashMap::new(),
        }
    }

    fn register(&mut self, mask: &Mask) {
        for block in mask.blocks.iter() {
            let value: usize = block.splitn(2, " = ").nth(1).unwrap().parse().unwrap();
            let register: usize = block
                .splitn(2, "]")
                .nth(0)
                .unwrap()
                .splitn(2, "[")
                .nth(1)
                .unwrap()
                .parse()
                .unwrap();

            self.memory.insert(register, mask.get_value(value));
        }
    }

    fn register_range(&mut self, mask: &Mask) {
        for block in mask.blocks.iter() {
            let value: usize = block.splitn(2, " = ").nth(1).unwrap().parse().unwrap();
            let mut register: usize = block
                .splitn(2, "]")
                .nth(0)
                .unwrap()
                .splitn(2, "[")
                .nth(1)
                .unwrap()
                .parse()
                .unwrap();

            for i in 0..mask.mask.len() {
                if mask.mask[i] == '1' {
                    register = register | 1 << (35 - i);
                } else if mask.mask[i] == 'X' {
                    register = register & (std::usize::MAX ^ 1 << (35 - i));
                }
            }

            let mut registers: HashSet<usize> = HashSet::new();
            registers.insert(register);

            for i in 0..mask.mask.len() {
                if mask.mask[i] == 'X' {
                    for entry in registers.clone().iter() {
                        registers.insert(entry | 1 << (35 - i));
                    }
                }
            }

            for register in registers {
                self.memory.insert(register, value);
            }
        }
    }
}

fn silver(masks: &Vec<Mask>) -> usize {
    let mut computer = Computer::new();

    for mask in masks {
        computer.register(mask);
    }

    computer
        .memory
        .iter()
        .fold(0usize, |sum, (_, value)| sum + value)
}

fn gold(masks: &Vec<Mask>) -> usize {
    let mut computer = Computer::new();

    for mask in masks {
        computer.register_range(mask);
    }

    computer
        .memory
        .iter()
        .fold(0usize, |sum, (_, value)| sum + value)
}

fn main() {
    let now = Instant::now();

    let input: Vec<&str> = include_str!("input").trim().split('\n').collect();

    let mut blocks: Vec<Mask> = Vec::new();
    let mut mask = Mask::new(input[0]);

    for line in input[1..].iter() {
        if line.contains("mask") {
            blocks.push(mask.clone());
            mask = Mask::new(line);
        } else {
            mask.blocks.push(line.to_string());
        }
    }

    blocks.push(mask);

    println!("Silver: {}", silver(&blocks));
    println!("Gold: {}", gold(&blocks));

    println!("Time: {}ms", now.elapsed().as_millis());
}
