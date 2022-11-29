extern crate nalgebra as na;

use na::Matrix5;
use std::fs::{self};

type Board = Matrix5<u32>;

fn read_numbers(filename: &str) -> Vec<u32> {
    fs::read_to_string(filename)
        .expect("File not found")
        .split_whitespace()
        .flat_map(|s| s.split(','))
        .map(|s| s.parse::<u32>().unwrap())
        .collect()
}

fn read_boards(raw_numbers: Vec<u32>) -> Vec<Board> {
    raw_numbers
        .chunks(25)
        .map(|chunk| Matrix5::from_vec(chunk.to_vec()).transpose())
        .collect()
}

fn print_board(board: &Board) {
    for row in board.row_iter() {
        for n in row.iter() {
            print!("{:2} ", n);
        }
        println!();
    }
    println!();
}

fn check(board: &Board, nums: &[u32]) -> bool {
    // check rows
    board
        .row_iter()
        .any(|row| row.iter().all(|n| nums.contains(n))) ||
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

fn part1() {
    let numbers = read_numbers("input_numbers.txt");
    let boards = read_boards(read_numbers("input_boards.txt"));

    for (i, val) in numbers.iter().enumerate() {
        for (_, board) in boards.iter().enumerate() {
            if check(board, &numbers[0..=i]) {
                let winning_numer = get_unmarked(board, &numbers[0..=i]).iter().sum::<u32>() * val;
                println!("Part 1: {}", winning_numer);
                return;
            }
        }
    }
}

fn main() {
    part1();
}
