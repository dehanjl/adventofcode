use crate::register_day;
use itertools::Itertools;
use std::collections::HashMap;

type Stack = HashMap<usize, Vec<char>>;
type Moves = Vec<(u32, usize, usize)>;

fn parse_input(input: &str) -> (Stack, Moves) {
    let moves = input
        .lines()
        .filter(|&l| l.starts_with("move"))
        .filter_map(|line| {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 6 {
                let qty = parts[1].parse().ok()?;
                let src: usize = parts[3].parse().ok()?;
                let dst: usize = parts[5].parse().ok()?;
                Some((qty, src - 1, dst - 1))
            } else {
                None
            }
        })
        .collect();

    let mut stacks = input
        .lines()
        .take_while(|&l| !l.starts_with("move"))
        .flat_map(|l| {
            l.chars()
                .skip(1)
                .step_by(4)
                .enumerate()
                .filter(|&(_, c)| c.is_alphabetic())
        })
        .into_group_map();

    stacks.iter_mut().for_each(|(_, v)| v.reverse());

    (stacks, moves)
}

fn part1(input: &str) {
    let (mut stacks, moves) = parse_input(input);

    moves.iter().for_each(|(qty, src, dst)| {
        (0..*qty).for_each(|_| {
            let cat = stacks.get_mut(src).unwrap().pop().unwrap();
            stacks.get_mut(dst).unwrap().push(cat)
        });
    });

    let mut result = String::new();
    for i in 0..stacks.len() {
        result.push(*stacks[&i].last().unwrap());
    }
    println!("Day 5 Part 1: {result}");
}

fn part2(input: &str) {
    let (mut stacks, moves) = parse_input(input);

    moves.iter().for_each(|(qty, src, dst)| {
        let l = stacks[src].len();
        let x: Vec<char> = stacks
            .get_mut(src)
            .unwrap()
            .drain(l - *qty as usize..)
            .collect();
        stacks.get_mut(dst).unwrap().extend(x);
    });

    let mut result = String::new();
    for i in 0..stacks.len() {
        result.push(*stacks[&i].last().unwrap());
    }
    println!("Day 5 Part 2: {result}");
}

register_day!(2022, 5, part1, part2);
