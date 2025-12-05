use crate::register_day;

fn parse_input(input: &str) -> Vec<Vec<i64>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as i64)
                .collect()
        })
        .collect()
}

fn part1(input: &str) {
    let mut total = 0;
    for seq in parse_input(input) {
        // Find the first occurrence of the maximum value.
        // We compare by value, and on ties we prefer the smaller index (the first one).
        let (idx1, val1) = seq[..seq.len() - 1]
            .iter()
            .enumerate()
            .max_by_key(|&(i, &v)| (v, std::cmp::Reverse(i)))
            .unwrap();

        let val2 = seq[idx1 + 1..].iter().max().unwrap();

        total += format!("{}{}", val1, val2).parse::<i64>().unwrap();
    }

    println!("Day 3 Part 1: {}", total);
}

fn part2(input: &str) {
    let mut total = 0;

    for seq in parse_input(input) {
        let mut collector: Vec<i64> = vec![];
        let mut start_idx = 0;

        for it in 0..12 {
            let end_idx = seq.len() - (11 - it);

            let (idx, val) = seq[start_idx..end_idx]
                .iter()
                .enumerate()
                .max_by_key(|&(i, &v)| (v, std::cmp::Reverse(i)))
                .unwrap();
            collector.push(*val);
            start_idx += idx + 1;
        }

        total += collector
            .iter()
            .map(|v| v.to_string())
            .collect::<String>()
            .parse::<i64>()
            .unwrap();
    }

    println!("Day 3 Part 2: {}", total);
}

register_day!(2025, 3, part1, part2);
