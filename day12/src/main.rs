use colored::*;
use itertools::Itertools;
use regex::Regex;
use std::cmp;
use std::collections::HashMap;
use std::collections::HashSet;
use std::time::Instant;

#[derive(Clone, Debug, Copy)]
struct Ship {
    n: i32,
    e: i32,
    rotation: i32,
    waypoint: (i32, i32),
}

impl Ship {
    fn new() -> Ship {
        Ship {
            n: 0,
            e: 0,
            rotation: 90,
            waypoint: (1, 10),
        }
    }

    fn step(&mut self, instruction: &str) {
        let value: i32 = instruction[1..].parse().unwrap();
        let c = instruction.chars().nth(0).unwrap();

        match c {
            'L' => self.rotate(-value),
            'R' => self.rotate(value),
            'F' => self.forward(value),
            _ => self.directional(value, c),
        };
    }

    fn step_waypoint(&mut self, instruction: &str) {
        let value: i32 = instruction[1..].parse().unwrap();
        let c = instruction.chars().nth(0).unwrap();

        match c {
            'L' => self.rotate(-value),
            'R' => self.rotate(value),
            'F' => self.forward_waypoint(value),
            _ => self.adjust_waypoint(value, c),
        }
    }

    fn rotate(&mut self, degrees: i32) {
        let degrees = ((degrees + 360) % 360) as f64;
        let radians = degrees.to_radians();

        let cos = radians.cos() as i32;
        let sin = radians.sin() as i32;

        let (n, e) = self.waypoint;

        let x = n * cos - e * sin;
        let y = e * cos + n * sin;

        self.waypoint = (x, y);

        self.rotation = (self.rotation + degrees as i32) % 360;
    }

    fn forward(&mut self, steps: i32) {
        match self.rotation {
            0 => self.directional(steps, 'N'),
            90 => self.directional(steps, 'E'),
            180 => self.directional(steps, 'S'),
            270 => self.directional(steps, 'W'),
            _ => unreachable!(),
        };
    }

    fn forward_waypoint(&mut self, steps: i32) {
        self.n = self.n + self.waypoint.0 * steps;
        self.e = self.e + self.waypoint.1 * steps;
    }

    fn adjust_waypoint(&mut self, steps: i32, direction: char) {
        let adjustment = match direction {
            'N' => (steps, 0),
            'E' => (0, steps),
            'S' => (-steps, 0),
            'W' => (0, -steps),
            _ => unreachable!(),
        };

        self.waypoint = (
            self.waypoint.0 + adjustment.0,
            self.waypoint.1 + adjustment.1,
        );
    }

    fn directional(&mut self, steps: i32, direction: char) {
        match direction {
            'N' => self.n = self.n + steps,
            'S' => self.n = self.n - steps,
            'E' => self.e = self.e - steps,
            'W' => self.e = self.e + steps,
            _ => unreachable!(),
        }
    }

    fn distance(&self) -> i32 {
        self.n.abs() + self.e.abs()
    }
}

fn silver(input: &Vec<&str>) -> i32 {
    let mut ship = Ship::new();

    input.iter().for_each(|x| ship.step(x));

    ship.distance()
}

fn gold(input: &Vec<&str>) -> i32 {
    let mut ship = Ship::new();
    input.iter().for_each(|x| ship.step_waypoint(x));

    ship.distance()
}

fn main() {
    let now = Instant::now();

    let input: Vec<&str> = include_str!("input").trim().split('\n').collect();

    println!("Silver: {}", silver(&input));
    println!("Silver: {}", gold(&input));

    println!("Time: {}ms", now.elapsed().as_millis());
}
