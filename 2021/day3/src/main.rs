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

fn count_bits(input: &Vec<Vec<u32>>) -> Vec<(u32, u32)> {
    let mut counts = vec![(0, 0); input[0].len()];

    for i in 0..input[0].len() {
        counts[i] = input
            .iter()
            .fold((0, 0), |(zero_count, one_count), line| match line[i] {
                0 => (zero_count + 1, one_count),
                1 => (zero_count, one_count + 1),
                _ => (zero_count, one_count),
            });
    }

    counts
}

fn most_common(counts: &(u32, u32)) -> u32 {
    if counts.1 >= counts.0 {
        1
    } else {
        0
    }
}

fn part1(input: &Vec<Vec<u32>>) -> u32 {
    let inner_len = input[0].len();
    let mask: u32 = (1 << inner_len) - 1;

    let gamma = count_bits(input)
        .iter()
        .map(most_common)
        .fold(0, |acc, bit| acc << 1 | bit);

    let epsilon = gamma ^ mask; // epsilon is the flipped value of gamma
    println!("Gamma: {} -> {:#b}", gamma, gamma);
    println!("Epsilon: {} -> {:#b}", epsilon, epsilon);

    gamma * epsilon
}

fn determine_rating(input: &Vec<Vec<u32>>, mask_fn: fn(&Vec<Vec<u32>>) -> Vec<u32>) -> u32 {
    let inner_len = input[0].len();

    let mut vec = input.clone();

    // filter the values in the input based on the most or least common bit
    for i in 0..inner_len {
        let mask = mask_fn(&vec);
        vec = vec.into_iter().filter(|p| p[i] == mask[i]).collect();

        if vec.len() == 1 {
            break;
        }
    }

    let rating = vec[0].iter().fold(0, |acc, &bit| acc << 1 | bit);
    println!("Rating: {} -> {:#b}", rating, rating);

    rating
}

fn part2(input: &Vec<Vec<u32>>) -> u32 {
    let ox_rating = determine_rating(input, |mask_in: &Vec<Vec<u32>>| {
        count_bits(mask_in).iter().map(most_common).collect()
    });
    let co_rating = determine_rating(input, |mask_in: &Vec<Vec<u32>>| {
        count_bits(mask_in)
            .iter()
            .map(most_common)
            .map(|x| x ^ 1)
            .collect()
    });

    ox_rating * co_rating
}

fn main() {
    let input = read_file("input.txt");

    println!("Part 1: {:?}", part1(&input));
    println!("Part 2: {:?}", part2(&input));
}
