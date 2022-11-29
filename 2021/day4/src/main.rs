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

fn check(board: &Board, nums: &[u32]) {
    // check rows
    for row in board.row_iter() {
        let x = row.iter().filter(|n| nums.contains(*n)).count();
        println!("row: {}", x);
    }
}

fn main() {
    println!("{:?}", read_numbers("numbers_example.txt"));

    let boards = read_boards(read_numbers("boards_example.txt"));
    boards.iter().for_each(print_board);
    boards.iter().for_each(|b| check(b, &[22, 13, 17, 11, 0]));
}
