use crate::register_day;

fn parse_input(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect()
}

fn count_bits(input: &[Vec<u32>]) -> Vec<(u32, u32)> {
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

fn part1(input: &str) {
    let data = parse_input(input);
    let inner_len = data[0].len();
    let mask: u32 = (1 << inner_len) - 1;

    let gamma = count_bits(&data)
        .iter()
        .map(most_common)
        .fold(0, |acc, bit| acc << 1 | bit);

    let epsilon = gamma ^ mask;
    let result = gamma * epsilon;
    println!("Day 3 Part 1: {result}");
}

fn determine_rating(input: &[Vec<u32>], mask_fn: fn(&[Vec<u32>]) -> Vec<u32>) -> u32 {
    let inner_len = input[0].len();
    let mut vec = input.to_vec();

    for i in 0..inner_len {
        let mask = mask_fn(&vec);
        vec = vec.into_iter().filter(|p| p[i] == mask[i]).collect();

        if vec.len() == 1 {
            break;
        }
    }

    vec[0].iter().fold(0, |acc, &bit| acc << 1 | bit)
}

fn part2(input: &str) {
    let data = parse_input(input);

    let ox_rating = determine_rating(&data, |mask_in: &[Vec<u32>]| {
        count_bits(mask_in).iter().map(most_common).collect()
    });
    let co_rating = determine_rating(&data, |mask_in: &[Vec<u32>]| {
        count_bits(mask_in)
            .iter()
            .map(most_common)
            .map(|x| x ^ 1)
            .collect()
    });

    let result = ox_rating * co_rating;
    println!("Day 3 Part 2: {result}");
}

register_day!(2021, 3, part1, part2);
