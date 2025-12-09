use itertools::Itertools;

use crate::{register_day, utils::Loc};

fn parse_input(input: &str) -> Vec<Loc> {
    input
        .lines()
        .map(|line| sscanf::scanf!(line, "{},{}", isize, isize).unwrap())
        .map(|(c, r)| Loc(r, c))
        .collect()
}

fn part1(input: &str) {
    let tiles = parse_input(input);

    let area = tiles
        .iter()
        .tuple_combinations()
        .map(|(loc1, loc2)| (loc1.0.abs_diff(loc2.0) + 1) * (loc1.1.abs_diff(loc2.1) + 1))
        .max()
        .unwrap();

    println!("Day 9 Part 1: {}", area);
}

fn _part2(input: &str) {
    let _data = parse_input(input);
    println!("Day 9 Part 2: {}", "TODO");
}

register_day!(2025, 9, part1);
