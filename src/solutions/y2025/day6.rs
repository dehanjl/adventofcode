use grid::Grid;

use crate::{register_day, utils::GridUtils};

fn parse_input(input: &str) -> (Grid<u64>, Vec<char>) {
    let (num_lines, ops_line) = input.trim().rsplit_once('\n').unwrap();

    let grid: Grid<u64> = num_lines
        .lines()
        .map(|line| {
            line.split_whitespace()
                .filter_map(|s| s.parse::<u64>().ok())
                .collect::<Vec<u64>>()
        })
        .collect::<Vec<Vec<u64>>>()
        .into();

    let ops: Vec<char> = ops_line
        .split_whitespace()
        .filter_map(|s| s.chars().next())
        .collect();
    (grid, ops)
}

fn part1(input: &str) {
    let (grid, ops) = parse_input(input);

    let total = grid
        .iter_cols()
        .enumerate()
        .map(|(col_idx, col)| match ops[col_idx] {
            '+' => col.sum::<u64>(),
            '*' => col.product::<u64>(),
            _ => 0,
        })
        .sum::<u64>();

    println!("Day 6 Part 1: {}", total);
}

fn parse_input2(input: &str) -> (Vec<Vec<u64>>, Vec<char>) {
    let (num_lines, ops_line) = input.trim().rsplit_once('\n').unwrap();
    let char_grid: Grid<char> = Grid::parse(num_lines);

    let mut nums: Vec<Vec<u64>> = vec![];
    let mut nums_inner: Vec<u64> = vec![];
    for col in char_grid.iter_cols() {
        if let Some(num) = col
            .filter(|c| c.is_digit(10))
            .collect::<String>()
            .parse::<u64>()
            .ok()
        {
            nums_inner.push(num);
        } else {
            nums.push(nums_inner.clone());
            nums_inner.clear();
        }
    }
    nums.push(nums_inner.clone());

    let ops: Vec<char> = ops_line
        .split_whitespace()
        .filter_map(|s| s.chars().next())
        .collect();

    (nums, ops)
}

fn part2(input: &str) {
    let (nums, ops) = parse_input2(input);

    let total = nums
        .into_iter()
        .enumerate()
        .map(|(col_idx, col)| match ops[col_idx] {
            '+' => col.into_iter().sum::<u64>(),
            '*' => col.into_iter().product::<u64>(),
            _ => 0,
        })
        .sum::<u64>();

    println!("Day 6 Part 2: {}", total);
}

register_day!(2025, 6, part1, part2);
