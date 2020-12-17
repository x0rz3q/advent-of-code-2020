#[macro_use]
extern crate itertools;

use colored::*;
use itertools::Itertools;
use regex::Regex;
use std::cmp;
use std::collections::HashMap;
use std::collections::HashSet;
use std::time::Instant;

#[derive(Hash, Clone, Debug, Copy)]
struct Coordinate {
    x: i64,
    y: i64,
    z: i64,
    w: i64,
}

impl Coordinate {
    fn new(x: i64, y: i64, z: i64, w: i64) -> Coordinate {
        Coordinate { x, y, z, w }
    }
}

impl Eq for Coordinate {}

impl PartialEq for Coordinate {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z && self.w == other.w
    }
}

#[derive(Clone, Debug)]
struct Map {
    coordinates: HashMap<Coordinate, bool>,
    max: (i64, i64, i64, i64),
    min: (i64, i64, i64, i64),
}

impl Map {
    fn new() -> Map {
        Map {
            coordinates: HashMap::new(),
            max: (0, 0, 0, 0),
            min: (0, 0, 0, 0),
        }
    }

    fn print(&self) {
        let (xmin, ymin, zmin, wmin) = self.min;
        let (xmax, ymax, zmax, wmax) = self.max;

        for z in zmin..=zmax {
            println!("z={}", z);
            for y in ymin..=ymax {
                for x in xmin..=xmax {
                    for w in wmin..wmax {
                        let coordinate = Coordinate::new(x, y, z, w);
                        let active = self.coordinates.get(&coordinate).unwrap_or(&false);

                        if *active {
                            print!("#");
                        } else {
                            print!(".");
                        }
                    }
                }

                println!("");
            }
        }
    }

    fn count_neighbors(&self, coordinate: Coordinate) -> usize {
        iproduct!(-1i64..=1i64, -1i64..=1i64, -1i64..=1i64)
            .filter(|(x, y, z)| !(*x == 0 && *y == 0 && *z == 0))
            .filter(|(x, y, z)| {
                *self
                    .coordinates
                    .get(&Coordinate::new(
                        *x + coordinate.x,
                        *y + coordinate.y,
                        *z + coordinate.z,
                        0,
                    ))
                    .unwrap_or(&false)
            })
            .count()
    }

    fn count_neighbors4d(&self, coordinate: Coordinate) -> usize {
        iproduct!(-1i64..=1i64, -1i64..=1i64, -1i64..=1i64, -1i64..=1i64)
            .filter(|(x, y, z, w)| !(*x == 0 && *y == 0 && *z == 0 && *w == 0))
            .filter(|(x, y, z, w)| {
                *self
                    .coordinates
                    .get(&Coordinate::new(
                        *x + coordinate.x,
                        *y + coordinate.y,
                        *z + coordinate.z,
                        *w + coordinate.w,
                    ))
                    .unwrap_or(&false)
            })
            .count()
    }

    fn cycle(&mut self) {
        // Inject new entries that we can possibly find
        let (xmin, ymin, zmin, _) = self.min;
        let (xmax, ymax, zmax, _) = self.max;

        self.max = (xmax + 1, ymax + 1, zmax + 1, 0);
        self.min = (xmin - 1, ymin - 1, zmin - 1, 0);

        let mut reference = self.coordinates.clone();
        for z in self.min.2..=self.max.2 {
            for y in self.min.1..=self.max.1 {
                for x in self.min.0..=self.max.1 {
                    let coordinate = Coordinate::new(x, y, z, 0);
                    let count = self.count_neighbors(coordinate.clone());
                    let activated = *reference.get(&coordinate).unwrap_or(&false);

                    if activated && count != 2 && count != 3 {
                        reference.insert(coordinate, false);
                    }

                    if !activated && count == 3 {
                        reference.insert(coordinate, true);
                    }
                }
            }
        }

        self.coordinates = reference.clone();
    }

    fn cycle4d(&mut self) {
        // Inject new entries that we can possibly find
        let (xmin, ymin, zmin, wmin) = self.min;
        let (xmax, ymax, zmax, wmax) = self.max;

        self.max = (xmax + 1, ymax + 1, zmax + 1, wmax + 1);
        self.min = (xmin - 1, ymin - 1, zmin - 1, wmin - 1);

        let mut reference = self.coordinates.clone();
        for w in self.min.3..=self.max.3 {
            for z in self.min.2..=self.max.2 {
                for y in self.min.1..=self.max.1 {
                    for x in self.min.0..=self.max.1 {
                        let coordinate = Coordinate::new(x, y, z, w);
                        let count = self.count_neighbors4d(coordinate.clone());
                        let activated = *reference.get(&coordinate).unwrap_or(&false);

                        if activated && count != 2 && count != 3 {
                            reference.insert(coordinate, false);
                        }

                        if !activated && count == 3 {
                            reference.insert(coordinate, true);
                        }
                    }
                }
            }
        }

        self.coordinates = reference.clone();
    }

    fn count(&self) -> usize {
        self.coordinates.iter().filter(|(_, value)| **value).count()
    }
}

fn silver(map: &mut Map) -> usize {
    for _ in 0..6 {
        map.cycle();
    }

    map.count()
}

fn gold(map: &mut Map) -> usize {
    for _ in 0..6 {
        map.cycle4d();
    }

    map.count()
}

fn main() {
    let now = Instant::now();

    let input: Vec<&str> = include_str!("input").trim().split('\n').collect();

    let mut y = 0;
    let z = 0;
    let mut map = Map::new();
    let mut x = 0;

    for entry in input {
        x = 0;
        for c in entry.chars() {
            let coordinate = Coordinate::new(x, y, z, 0);

            map.coordinates.insert(coordinate, c == '#');
            x += 1;
        }

        y += 1;
    }

    map.max = (x - 1, y - 1, 0, 0);
    println!("Silver: {}", silver(&mut map.clone()));
    println!("Gold: {}", gold(&mut map.clone()));

    println!("Time: {}ms", now.elapsed().as_millis());
}
