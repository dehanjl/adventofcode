use std::{
    fs::File,
    io::{BufRead, BufReader},
};

type Point = Vec<u32>;
type Line = Vec<Point>;
type Board = Vec<Vec<u32>>;

// a function to read a file by lines
fn read_input(filename: &str) -> Vec<Line> {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    reader
        .lines()
        .map(|line| {
            line.unwrap()
                .split(" -> ")
                .map(str::trim)
                .map(String::from)
                .map(|s| {
                    s.split(',')
                        .map(|s| s.parse::<u32>().unwrap())
                        .collect::<Point>()
                })
                .collect::<Line>()
        })
        .collect::<Vec<Line>>()
}

fn part1() {
    let lines = read_input("input_example.txt");

    let size = *lines.iter().flatten().flatten().max().unwrap() as usize + 1;
    let mut board: Board = vec![vec![0; size]; size];

    for line in board {
        println!("{:?}", line);
    }
}

fn main() {
    part1();
}
