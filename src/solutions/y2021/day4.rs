use crate::register_day;
use nalgebra::Matrix5;

type Board = Matrix5<u32>;

fn parse_input(input: &str) -> (Vec<u32>, Vec<Board>) {
    let mut lines = input.lines();

    // First line contains the drawn numbers
    let numbers: Vec<u32> = lines
        .next()
        .unwrap()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect();

    // Rest contains boards separated by blank lines
    let board_numbers: Vec<u32> = lines
        .flat_map(|line| line.split_whitespace())
        .map(|s| s.parse().unwrap())
        .collect();

    let boards = board_numbers
        .chunks(25)
        .map(|chunk| Matrix5::from_vec(chunk.to_vec()).transpose())
        .collect();

    (numbers, boards)
}

fn check(board: &Board, nums: &[u32]) -> bool {
    // check rows
    board
        .row_iter()
        .any(|row| row.iter().all(|n| nums.contains(n)))
        ||
    // check columns
    board
        .column_iter()
        .any(|col| col.iter().all(|n| nums.contains(n)))
}

fn get_unmarked(board: &Board, nums: &[u32]) -> Vec<u32> {
    board
        .iter()
        .filter(|n| !nums.contains(n))
        .cloned()
        .collect()
}

fn part1(input: &str) {
    let (numbers, boards) = parse_input(input);

    for (i, val) in numbers.iter().enumerate() {
        for board in boards.iter() {
            if check(board, &numbers[0..=i]) {
                let result = get_unmarked(board, &numbers[0..=i]).iter().sum::<u32>() * val;
                println!("Day 4 Part 1: {result}");
                return;
            }
        }
    }
}

fn part2(input: &str) {
    let (numbers, mut boards) = parse_input(input);

    for (i, val) in numbers.iter().enumerate() {
        if boards.len() > 1 {
            let winners: Vec<&Board> = boards
                .iter()
                .filter(|board| check(board, &numbers[0..=i]))
                .collect();
            if winners.len() == boards.len() {
                let result = get_unmarked(winners[0], &numbers[0..=i]).iter().sum::<u32>() * val;
                println!("Day 4 Part 2: {result}");
                return;
            }
            boards.retain(|board| !check(board, &numbers[0..=i]));
        } else {
            let board = &boards[0];
            if check(board, &numbers[0..=i]) {
                let result = get_unmarked(board, &numbers[0..=i]).iter().sum::<u32>() * val;
                println!("Day 4 Part 2: {result}");
                return;
            }
        }
    }
}

register_day!(2021, 4, part1, part2);
