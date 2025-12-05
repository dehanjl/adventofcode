use crate::register_day;
use crate::utils::{GridUtils, Loc, DIR8};
use grid::Grid;

fn parse_input(input: &str) -> Grid<char> {
    Grid::parse(input)
}

fn adjacent_count(grid: &Grid<char>, loc: Loc) -> usize {
    DIR8.into_iter()
        .map(|dir| loc + dir)
        .filter_map(|nloc| grid.get(nloc.0, nloc.1).copied().filter(|c| *c == '@'))
        .count()
}

fn part1(input: &str) {
    let grid = parse_input(input);

    let count = grid
        .indexed_iter()
        .filter(|(_, c)| **c == '@')
        .filter(|((lr, lc), _)| {
            let loc = Loc(*lr as isize, *lc as isize);
            adjacent_count(&grid, loc) < 4
        })
        .count();

    println!("Day 4 Part 1: {}", count);
}

fn part2(input: &str) {
    let mut grid = parse_input(input);
    let mut count = 0;

    loop {
        let mut collector: Vec<Loc> = vec![];

        for ((lr, lc), _) in grid.indexed_iter().filter(|(_, c)| **c == '@') {
            let loc = Loc(lr as isize, lc as isize);

            if adjacent_count(&grid, loc) < 4 {
                collector.push(loc);
            }
        }

        if collector.is_empty() {
            break;
        }
        count += collector.len();

        for loc in collector {
            if let Some(c) = grid.get_mut(loc.0 as usize, loc.1 as usize) {
                *c = 'x';
            }
        }
    }

    println!("Day 4 Part 2: {}", count);
}

register_day!(2025, 4, part1, part2);
