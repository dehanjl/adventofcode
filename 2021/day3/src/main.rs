use std::{
    fs::File,
    io::{prelude::*, BufReader},
};

#[derive(Clone, Debug)]
enum Common {
    ZERO,
    ONE,
    EQUAL(u32),
}

impl Common {
    fn value(&self) -> u32 {
        match self {
            Common::ZERO => 0,
            Common::ONE => 1,
            Common::EQUAL(x) => *x,
        }
    }
}

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
    let inner_len = input[0].len();
    let mut counts = vec![(0, 0); inner_len];

    for i in 0..inner_len {
        let (mut zero_count, mut one_count) = (0, 0);

        input.iter().for_each(|line| match line[i] {
            0 => zero_count += 1,
            1 => one_count += 1,
            _ => panic!("Invalid input"),
        });

        counts[i] = (zero_count, one_count);
    }

    counts
}

fn most_common_bits(input: &Vec<Vec<u32>>) -> Vec<Common> {
    let counts = count_bits(input);
    let inner_len = counts.len();
    let mut most_common = vec![Common::EQUAL(1); inner_len];

    for i in 0..inner_len {
        let (zero_count, one_count) = counts[i];

        most_common[i] = if zero_count > one_count {
            Common::ZERO
        } else if zero_count < one_count {
            Common::ONE
        } else {
            Common::EQUAL(1)
        };
    }

    most_common
}

fn least_common_bits(input: &Vec<Vec<u32>>) -> Vec<Common> {
    let counts = count_bits(input);
    let inner_len = counts.len();
    let mut least_common = vec![Common::EQUAL(0); inner_len];

    for i in 0..inner_len {
        let (zero_count, one_count) = counts[i];

        least_common[i] = if zero_count < one_count {
            Common::ZERO
        } else if zero_count > one_count {
            Common::ONE
        } else {
            Common::EQUAL(0)
        };
    }

    least_common
}

fn part1(input: &Vec<Vec<u32>>) -> u32 {
    let inner_len = input[0].len();
    let mask: u32 = (1 << inner_len) - 1;

    let gamma = most_common_bits(input)
        .iter()
        .map(|b| b.value())
        .fold(0, |acc, bit| acc << 1 | bit);

    let epsilon = gamma ^ mask; // epsilon is the flipped value of gamma
    println!("Gamma: {} -> {:#b}", gamma, gamma);
    println!("Epsilon: {} -> {:#b}", epsilon, epsilon);

    gamma * epsilon
}

fn determine_rating(
    input: &Vec<Vec<u32>>,
    mask_common_func: fn(&Vec<Vec<u32>>) -> Vec<Common>,
) -> u32 {
    let inner_len = input[0].len();

    let mut vec = input.clone();

    // filter the values in the input based on the most or least common bit
    for i in 0..inner_len {
        let mask = mask_common_func(&vec);

        vec = vec
            .into_iter()
            .filter(|p| p[i] == mask[i].value())
            .collect();

        // println!("i={} Mask={} Vec: {:?}", i, mask[i].value(), vec);

        if vec.len() == 1 {
            break;
        }
    }

    let rating = vec[0].iter().fold(0, |acc, &bit| acc << 1 | bit);
    println!("Rating: {} -> {:#b}", rating, rating);

    rating
}

fn part2(input: &Vec<Vec<u32>>) -> u32 {
    let ox_rating = determine_rating(input, most_common_bits);
    let co_rating = determine_rating(input, least_common_bits);

    ox_rating * co_rating
}

fn main() {
    let input = read_file("input.txt");
    println!("Most Common:: {:?}", most_common_bits(&input));

    println!("Part 1: {:?}", part1(&input));
    println!("Part 2: {:?}", part2(&input));
}
