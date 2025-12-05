use crate::register_day;

fn parse_input(input: &str) -> Vec<(i64, i64)> {
    input
        .trim()
        .split(",")
        .map(|range| sscanf::sscanf!(range, "{i64}-{i64}").unwrap())
        .collect()
}

fn part1(input: &str) {
    let mut total = 0;
    for num in parse_input(input)
        .into_iter()
        .flat_map(|(start, end)| start..=end)
    {
        let s = num.to_string();
        let (first_half, second_half) = s.split_at(s.len() / 2);
        if first_half == second_half {
            total += num;
        }
    }

    println!("Day 2 Part 1: {}", total);
}

fn substrings(s: &str) -> impl Iterator<Item = &str> {
    let half_len = s.len() / 2;
    (1..=half_len).map(move |i| &s[..i])
}

fn part2(input: &str) {
    let mut total = 0;
    for num in parse_input(input)
        .into_iter()
        .flat_map(|(start, end)| start..=end)
    {
        let s = num.to_string();
        for ss in substrings(&s) {
            let repeated = ss.repeat(s.len() / ss.len());
            if repeated == s {
                total += num;
                break;
            }
        }
    }

    println!("Day 2 Part 2: {}", total);
}

register_day!(2025, 2, part1, part2);
