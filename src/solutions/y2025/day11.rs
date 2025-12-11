use hashbrown::HashMap;

use crate::register_day;
use std::collections::VecDeque;

struct Node {
    id: String,
    outgoing: Vec<String>,
    incoming: Vec<String>,
}

type Graph = HashMap<String, Node>;

fn parse_input(input: &str) -> Graph {
    let mut graph: Graph = input
        .lines()
        .map(|line| {
            let (id_part, outgoing_part) = line.split_once(':').unwrap();
            let id = id_part.trim().to_string();
            let outgoing = outgoing_part
                .split_whitespace()
                .map(str::to_string)
                .collect();
            (
                id.clone(),
                Node {
                    id,
                    outgoing,
                    incoming: Vec::new(),
                },
            )
        })
        .collect();

    graph.insert(
        "out".to_string(),
        Node {
            id: "out".to_string(),
            outgoing: Vec::new(),
            incoming: Vec::new(),
        },
    );

    // second pass: build incoming edges
    let ids: Vec<String> = graph.keys().cloned().collect();
    for src in &ids {
        let outgoing = graph[src].outgoing.clone();
        for dst in outgoing {
            if let Some(target) = graph.get_mut(&dst) {
                target.incoming.push(src.clone());
            }
        }
    }

    graph
}

/// Perform a topological sort on the graph using Kahn's algorithm
fn topo_sort(graph: &Graph) -> Vec<String> {
    let mut in_degree: HashMap<String, usize> = graph
        .iter()
        .map(|(id, node)| (id.clone(), node.incoming.len()))
        .collect();

    let mut q: VecDeque<String> = in_degree
        .iter()
        .filter_map(|(id, &deg)| if deg == 0 { Some(id.clone()) } else { None })
        .collect();

    let mut sorted: Vec<String> = Vec::new();

    while let Some(node_id) = q.pop_front() {
        sorted.push(node_id.clone());
        for neighbor in &graph[&node_id].outgoing {
            if let Some(deg) = in_degree.get_mut(neighbor) {
                *deg -= 1;
                if *deg == 0 {
                    q.push_back(neighbor.clone());
                }
            }
        }
    }

    if sorted.len() != graph.len() {
        panic!("Graph has at least one cycle");
    }

    sorted
}

fn count_paths(graph: &Graph, sorted: &Vec<String>, start: &str, end: &str) -> u64 {
    let mut ways: HashMap<String, u64> = graph.keys().map(|id| (id.clone(), 0u64)).collect();
    ways.insert(start.to_string(), 1);

    for node_id in sorted {
        let count = ways[node_id];
        for neighbor in &graph[node_id].outgoing {
            *ways.get_mut(neighbor).unwrap() += count;
        }
    }

    ways.get(end).cloned().unwrap_or(0)
}

fn part1(input: &str) {
    let graph = parse_input(input);
    let sorted = topo_sort(&graph);

    let paths = count_paths(&graph, &sorted, "you", "out");

    println!("Day 11 Part 1: {}", paths);
}

fn part2(input: &str) {
    let graph = parse_input(input);
    let sorted = topo_sort(&graph);

    let prod = |path: &[(&str, &str)]| -> u64 {
        path.iter()
            .map(|(a, b)| count_paths(&graph, &sorted, a, b))
            .product()
    };

    // path type 1: svr -> dac -> fft -> ou
    let c1: u64 = prod(&[("svr", "dac"), ("dac", "fft"), ("fft", "out")]);

    // path type 2: svr -> fft -> dac -> out
    let c2: u64 = prod(&[("svr", "fft"), ("fft", "dac"), ("dac", "out")]);

    println!("Day 11 Part 2: {}", c1 + c2);
}

register_day!(2025, 11, part1, part2);
