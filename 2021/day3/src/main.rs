use std::{
    fs::File,
    io::{prelude::*, BufReader},
};

fn read_file(filepath: &str) -> Vec<Vec<u32>> {
    let file = File::open(filepath).expect("File not found");
    let reader = BufReader::new(file);

    reader
        .lines()
        .map(|line| {
            line.unwrap()
                .chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect()
        })
        .collect()
}

fn most_common_bits(input: &Vec<Vec<u32>>) -> Vec<u32> {
    let inner_len = input[0].len();
    let mut most_common = vec![0; inner_len];

    for i in 0..inner_len {
        let (mut zero_count, mut one_count) = (0, 0);

        input.iter().for_each(|line| match line[i] {
            0 => zero_count += 1,
            1 => one_count += 1,
            _ => panic!("Invalid input"),
        });

        most_common[i] = if zero_count > one_count { 0 } else { 1 };
    }

    most_common
}

fn part1(input: &Vec<Vec<u32>>) -> u32 {
    let inner_len = input[0].len();
    let mask: u32 = (1 << inner_len) - 1;

    let mut gamma: u32 = 0;

    most_common_bits(input)
        .iter()
        .enumerate()
        .for_each(|(i, bit)| {
            gamma |= bit << (inner_len - i - 1);
        });

    let epsilon = gamma ^ mask;
    println!("Gamma: {} -> {:#b}", gamma, gamma);
    println!("Epsilon: {} -> {:#b}", epsilon, epsilon);

    gamma * epsilon
}

fn main() {
    let input = read_file("input.txt");
    println!("Most Common:: {:?}", most_common_bits(&input));

    println!("Part 1: {:?}", part1(&input));
}
