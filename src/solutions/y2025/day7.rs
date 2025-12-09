use grid::Grid;
use hashbrown::{HashMap, HashSet};

use crate::{
    register_day,
    utils::{Dir, GridUtils, Loc},
};

fn parse_input(input: &str) -> (Grid<char>, HashSet<Loc>) {
    let manifold = Grid::parse(input);
    let beam_starts = manifold.find_set(|c| *c == 'S');
    (manifold, beam_starts)
}

fn part1(input: &str) {
    let (manifold, mut beam_heads) = parse_input(input);

    let mut split_count = 0;

    for _idx in 1..manifold.rows() {
        let mut next_beam_heads = HashSet::new();

        for &head in &beam_heads {
            match manifold.get_loc(&(head + Dir::South)) {
                Some('^') => {
                    split_count += 1;
                    next_beam_heads.insert(head + Dir::South + Dir::West);
                    next_beam_heads.insert(head + Dir::South + Dir::East);
                }
                Some(_) => {
                    next_beam_heads.insert(head + Dir::South);
                }
                None => {}
            }
        }
        beam_heads = next_beam_heads;
    }

    println!("Day 7 Part 1: {}", split_count);
}

fn part2(input: &str) {
    let (manifold, beam_heads) = parse_input(input);
    let mut beam_heads = HashMap::from_iter(beam_heads.into_iter().map(|loc| (loc, 1u64)));

    for _idx in 1..manifold.rows() {
        let mut next_beam_heads = HashMap::new();

        let mut add_beams = |loc: Loc, count: u64| {
            *next_beam_heads.entry(loc).or_insert(0) += count;
        };

        for (&loc, &count) in &beam_heads {
            match manifold.get_loc(&(loc + Dir::South)) {
                Some('^') => {
                    add_beams(loc + Dir::South + Dir::West, count);
                    add_beams(loc + Dir::South + Dir::East, count);
                }
                Some(_) => {
                    add_beams(loc + Dir::South, count);
                }
                None => {}
            }
        }
        beam_heads = next_beam_heads;
    }

    let timeline_count = beam_heads.values().sum::<u64>();

    println!("Day 7 Part 2: {}", timeline_count);
}

register_day!(2025, 7, part1, part2);
