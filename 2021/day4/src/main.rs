use std::fs::{self};

type Board = Vec<(u32, bool)>;

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
        .map(|c| c.to_vec().iter().map(|f| (*f, false)).collect())
        .collect()
}

fn get_rows(board: &Board) -> Vec<Vec<(u32, bool)>> {
    board.chunks(5).map(|c| c.to_vec()).collect()
}

fn get_cols(board: &Board) -> Vec<Vec<(u32, bool)>> {
    (0..5)
        .map(|col| {
            (0..25)
                .step_by(5)
                .map(|row| board[row + col])
                .collect::<Vec<(u32, bool)>>()
        })
        .collect::<Vec<Vec<(u32, bool)>>>()
}

fn print_board(board: &Board) {
    let rows = get_rows(board);
    for row in rows {
        for (num, _) in row {
            print!("{} ", num);
        }
        println!();
    }
    println!();
}

fn main() {
    println!("{:?}", read_numbers("numbers_example.txt"));

    let boards = read_boards(read_numbers("boards_example.txt"));
    boards.iter().for_each(print_board);
}
