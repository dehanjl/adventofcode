use crate::register_day;
use itertools::Itertools;

fn parse_input(input: &str) -> Vec<char> {
    input.chars().collect()
}

fn part1(input: &str) {
    let chars = parse_input(input);

    for i in 0..chars.len() - 3 {
        let slice = &chars[i..i + 4];
        if slice.iter().unique().count() == 4 {
            println!("Day 6 Part 1: {}", i + 4);
            return;
        }
    }
}

fn part2(input: &str) {
    let chars = parse_input(input);

    for i in 0..chars.len() - 13 {
        let slice = &chars[i..i + 14];
        if slice.iter().unique().count() == 14 {
            println!("Day 6 Part 2: {}", i + 14);
            return;
        }
    }
}

register_day!(2022, 6, part1, part2);
