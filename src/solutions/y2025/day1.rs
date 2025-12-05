use crate::register_day;

fn parse_input(input: &str) -> Vec<(char, i32)> {
    input
        .lines()
        .map(|line| sscanf::sscanf!(line, "{char}{i32}").unwrap())
        .collect()
}

fn part1(input: &str) {
    let count = parse_input(input)
        .into_iter()
        .scan(50, |pos, (dir, dist)| {
            *pos = (*pos + if dir == 'R' { dist } else { -dist }).rem_euclid(100);
            Some(*pos)
        })
        .filter(|&pos| pos == 0)
        .count();

    println!("Day 1 Part 1: {}", count);
}

fn part2(input: &str) {
    let (mut position, mut count) = (50, 0);
    for (dir, dist) in parse_input(input) {
        for _ in 0..dist {
            position = (position + if dir == 'R' { 1 as i32 } else { -1 as i32 }).rem_euclid(100);
            if position == 0 {
                count += 1;
            }
        }
    }

    println!("Day 1 Part 2: {}", count);
}

register_day!(2025, 1, part1, part2);
