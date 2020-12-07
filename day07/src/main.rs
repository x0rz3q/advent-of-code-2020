use regex::Regex;
use std::{
    collections::{HashMap, HashSet, VecDeque},
    time::Instant,
};

#[derive(Clone, Debug, Hash)]
struct Edge {
    name: String,
    weight: i32,
}

#[derive(Clone, Debug)]
struct Graph {
    nodes: HashSet<String>,
    edges: HashMap<String, Vec<Edge>>,
}

impl Graph {
    fn new() -> Graph {
        Graph {
            nodes: HashSet::new(),
            edges: HashMap::new(),
        }
    }

    fn add_node(&mut self, node: String) {
        if !self.nodes.contains(&node) {
            self.nodes.insert(node.clone());
            self.edges.insert(node.clone(), Vec::new());
        }
    }

    fn add_edge(&mut self, node: String, edge: String, weight: i32) {
        let edge = Edge {
            name: edge.clone(),
            weight,
        };

        self.add_node(edge.name.clone());
        self.edges.entry(node).or_insert(Vec::new()).push(edge);
    }
}

fn silver(input: &Graph) -> u32 {
    let mut count = 0;

    for node in input.nodes.iter() {
        if node == "shiny gold" {
            continue;
        }

        let mut queue = VecDeque::new();
        queue.push_back(node);

        while queue.len() > 0 {
            let node = queue.pop_front().unwrap();

            if node == "shiny gold" {
                count += 1;
                break;
            }

            for edge in input.edges.get(node).unwrap().iter() {
                queue.push_back(&edge.name);
            }
        }
    }

    count
}

fn walk(input: &Graph, node: String) -> i32 {
    let mut sum = 1;

    for edge in input.edges.get(&node).unwrap() {
        sum += walk(input, edge.name.clone()) * edge.weight;
    }

    sum
}

fn gold(input: &Graph) -> i32 {
    walk(input, "shiny gold".to_string()) - 1
}

fn create_graph(input: &Vec<String>) -> Graph {
    let mut graph = Graph::new();

    let re: Regex =
        Regex::new(r"(?P<node>\w+ \w+) bags contain (?P<weight>\d+) (?P<edge>\w+ \w+) bag")
            .unwrap();
    let re_no_bags: Regex = Regex::new(r"(?P<node>\w+ \w+) bags contain no other bags.").unwrap();
    let re_edge: Regex = Regex::new(r"(?P<weight>\d+) (?P<edge>\w+ \w+) bag").unwrap();

    for entry in input {
        let chunks: Vec<&str> = entry.split(", ").collect();

        if !re.is_match(chunks.get(0).unwrap()) {
            let caps = re_no_bags
                .captures(chunks.get(0).unwrap())
                .expect("Capture went wrong");
            let node = caps.name("node").unwrap().as_str().to_string();
            graph.add_node(node);

            continue;
        }

        let caps = re
            .captures(chunks.get(0).unwrap())
            .expect("Capture went wrong");
        let node = caps.name("node").unwrap().as_str().to_string();
        let weight: i32 = caps
            .name("weight")
            .unwrap()
            .as_str()
            .parse()
            .expect("Could not parse value");
        let edge = caps.name("edge").unwrap().as_str().to_string();

        graph.add_node(node.clone());
        graph.add_edge(node.clone(), edge, weight);

        for chunk in chunks.iter().skip(1) {
            let caps = re_edge.captures(chunk).expect("Capture went wrong");
            let weight: i32 = caps
                .name("weight")
                .unwrap()
                .as_str()
                .parse()
                .expect("Could not parse value");
            let edge = caps.name("edge").unwrap().as_str().to_string();

            graph.add_edge(node.clone(), edge, weight);
        }
    }

    graph
}

fn main() {
    let now = Instant::now();

    let input: Vec<String> = include_str!("input")
        .trim()
        .split('\n')
        .map(|x| x.to_string())
        .collect();

    let graph = create_graph(&input);

    println!("Silver: {}", silver(&graph));
    println!("Gold: {}", gold(&graph));

    println!("Time: {}ms", now.elapsed().as_millis());
}
