use std::time::Instant;

#[derive(Hash, Clone, Debug)]
struct Range {
    name: String,
    first: (u64, u64),
    second: (u64, u64),
}

impl Range {
    fn new(class: &str) -> Range {
        let name = class.splitn(2, ": ").nth(0).unwrap();
        let mut ranges = class.splitn(2, ": ").nth(1).unwrap().splitn(2, " or ");

        let mut parts = ranges.nth(0).unwrap().splitn(2, "-");
        let begin: u64 = parts.nth(0).unwrap().parse().unwrap();
        let end: u64 = parts.nth(0).unwrap().parse().unwrap();

        let first = (begin, end);

        let mut parts = ranges.nth(0).unwrap().splitn(2, "-");
        let begin: u64 = parts.nth(0).unwrap().trim().parse().unwrap();
        let end: u64 = parts.nth(0).unwrap().parse().unwrap();

        Range {
            name: name.to_string(),
            first: first,
            second: (begin, end),
        }
    }

    fn within(&self, number: u64) -> bool {
        (number >= self.first.0 && number <= self.first.1)
            || (number >= self.second.0 && number <= self.second.1)
    }
}

fn silver(ranges: &Vec<Range>, tickets: &Vec<Vec<u64>>) -> u64 {
    let mut error = 0;

    for ticket in tickets {
        for value in ticket {
            if !ranges.iter().any(|x| x.within(*value)) {
                error += value;
            }
        }
    }

    error
}

fn find_valid(valid: Vec<(u64, Vec<u64>)>, picked: Vec<(u64, u64)>) -> Option<Vec<(u64, u64)>> {
    if picked.len() == valid.len() {
        return Some(picked);
    }

    for candidate in valid.get(picked.len()).unwrap().1.clone() {
        if !picked.iter().any(|x| x.1 == candidate) {
            let mut picked = picked.clone();
            picked.push((valid.get(picked.len()).unwrap().0, candidate));

            let result = find_valid(valid.clone(), picked);

            if result.is_some() {
                return result;
            }
        }
    }

    None
}

fn gold(ranges: &Vec<Range>, tickets: &Vec<Vec<u64>>, mine: Vec<u64>) -> u64 {
    let mut valid = Vec::new();

    for ticket in tickets {
        if ticket.iter().all(|x| ranges.iter().any(|y| y.within(*x))) {
            valid.push(ticket);
        }
    }

    // index has options
    let mut entries: Vec<(u64, Vec<u64>)> = Vec::new();

    // Loop over all fields in the tickets.
    for y in 0..valid[0].len() {
        let mut options = Vec::new();

        for i in 0..ranges.len() {
            if valid.iter().all(|x| ranges[i].within(x[y])) {
                options.push(i as u64);
            }
        }

        entries.push((y as u64, options));
    }

    entries.sort_by(|x, y| x.1.len().cmp(&y.1.len()));

    let order = find_valid(entries, Vec::new()).unwrap();
    let mut result = 1;

    for entry in order.iter() {
        if ranges[entry.1 as usize].name.starts_with("departure") {
            result *= mine[entry.0 as usize];
        }
    }

    result
}

fn main() {
    let now = Instant::now();

    let blocks: Vec<&str> = include_str!("input").trim().split("\n\n").collect();

    let mut ranges: Vec<Range> = Vec::new();
    for range in blocks[0].split('\n') {
        ranges.push(Range::new(range));
    }

    let mine: Vec<u64> = blocks[1]
        .split("\n")
        .nth(1)
        .unwrap()
        .split(",")
        .map(|x| x.parse().unwrap())
        .collect();
    let tickets: Vec<Vec<u64>> = blocks[2]
        .split("\n")
        .skip(1)
        .map(|x| x.split(",").map(|x| x.parse().unwrap()).collect())
        .collect();

    println!("Silver: {}", silver(&ranges, &tickets));
    println!("Gold: {}", gold(&ranges, &tickets, mine));

    println!("Time: {}ms", now.elapsed().as_millis());
}
