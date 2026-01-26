use crate::register_day;
use std::cmp::Ordering;

fn parse_input(input: &str) -> Vec<(u32, u32, u32, u32)> {
    input
        .lines()
        .filter_map(|line| {
            let parts: Vec<u32> = line
                .split(|c| c == '-' || c == ',')
                .filter_map(|s| s.parse().ok())
                .collect();
            if parts.len() == 4 {
                Some((parts[0], parts[1], parts[2], parts[3]))
            } else {
                None
            }
        })
        .collect()
}

fn part1(input: &str) {
    let sum = parse_input(input)
        .iter()
        .map(|(r1, f1, r2, f2)| (r1.cmp(r2), f1.cmp(f2)))
        .filter(|(x, y)| {
            [x, y].iter().any(|&x| x == &Ordering::Equal)
                || matches!(
                    (x, y),
                    (Ordering::Less, Ordering::Greater) | (Ordering::Greater, Ordering::Less)
                )
        })
        .count();

    println!("Day 4 Part 1: {sum}");
}

fn part2(input: &str) {
    let sum = parse_input(input)
        .iter()
        .map(|(r1, f1, r2, f2)| (r1.cmp(r2), f1.cmp(f2), r1.cmp(f2), f1.cmp(r2)))
        .filter(|(a, b, c, d)| {
            [a, b, c, d].iter().any(|&x| x == &Ordering::Equal)
                || matches!(
                    (a, b, c, d),
                    (Ordering::Less, Ordering::Greater, _, _) // range 1 contains range 2
                        | (Ordering::Greater, Ordering::Less, _, _) // range 2 contains range 1
                        | (_, Ordering::Less, _, Ordering::Greater) // range 1 end is in range 2
                        | (Ordering::Greater, _, Ordering::Less, _) // range 1 start is in range 2
                )
        })
        .count();

    println!("Day 4 Part 2: {sum}");
}

register_day!(2022, 4, part1, part2);
