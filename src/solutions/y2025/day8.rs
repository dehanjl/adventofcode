use std::hash::Hash;

use hashbrown::HashMap;
use itertools::Itertools;
use ordered_float::OrderedFloat;

use crate::{Opt, register_day};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Loc3D {
    x: i32,
    y: i32,
    z: i32,
}

impl Loc3D {
    fn euclid_dist(&self, other: &Loc3D) -> f64 {
        let dx = (self.x - other.x) as f64;
        let dy = (self.y - other.y) as f64;
        let dz = (self.z - other.z) as f64;
        (dx * dx + dy * dy + dz * dz).sqrt()
    }
}

struct UnionFind {
    parent: Vec<usize>,
    size: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            size: vec![1; n],
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    fn union(&mut self, x: usize, y: usize) {
        let (mut px, mut py) = (self.find(x), self.find(y));
        if px == py {
            return; // already connected
        }

        // ensure px is the larger set
        if self.size[px] < self.size[py] {
            std::mem::swap(&mut px, &mut py);
        }

        self.parent[py] = px;
        self.size[px] += self.size[py];
    }

    fn all_connected(&mut self) -> bool {
        let root = self.find(0);
        for i in 1..self.parent.len() {
            if self.find(i) != root {
                return false;
            }
        }
        true
    }
}

fn parse_input(input: &str) -> Vec<Loc3D> {
    input
        .lines()
        .map(|line| sscanf::scanf!(line, "{},{},{}", i32, i32, i32).unwrap())
        .map(|(x, y, z)| Loc3D { x, y, z })
        .collect()
}

fn part1(input: &str) {
    let box_locs = parse_input(input);
    let num_merges = if Opt::get().real { 1000 } else { 10 };
    let closest_pairs = box_locs
        .clone()
        .into_iter()
        .tuple_combinations()
        .sorted_by_cached_key(|(loc1, loc2)| OrderedFloat(loc1.euclid_dist(loc2)))
        .take(num_merges)
        .collect::<Vec<(Loc3D, Loc3D)>>();

    let loc_to_idx: HashMap<&Loc3D, usize> = box_locs
        .iter()
        .enumerate()
        .map(|(i, loc)| (loc, i))
        .collect();
    let mut uf = UnionFind::new(box_locs.len());

    for (loc1, loc2) in &closest_pairs {
        uf.union(loc_to_idx[loc1], loc_to_idx[loc2]);
    }

    let prod: usize = (0..box_locs.len())
        .filter(|&i| uf.parent[i] == i) // roots only
        .map(|i| uf.size[i])
        .sorted()
        .rev()
        .take(3)
        .product();

    println!("Day 8 Part 1: {}", prod);
}

fn part2(input: &str) {
    let box_locs = parse_input(input);

    let loc_to_idx: HashMap<&Loc3D, usize> = box_locs
        .iter()
        .enumerate()
        .map(|(i, loc)| (loc, i))
        .collect();
    let mut uf = UnionFind::new(box_locs.len());

    let (loc_a, loc_b) = box_locs
        .clone()
        .into_iter()
        .tuple_combinations()
        .sorted_by_cached_key(|(loc1, loc2)| OrderedFloat(loc1.euclid_dist(loc2)))
        .find_map(|(loc1, loc2)| {
            uf.union(loc_to_idx[&loc1], loc_to_idx[&loc2]);
            if uf.all_connected() {
                return Some((loc1, loc2));
            }
            None
        })
        .expect("How did you get here?");

    println!("Day 8 Part 2: {}", loc_a.x * loc_b.x);
}

register_day!(2025, 8, part1, part2);
