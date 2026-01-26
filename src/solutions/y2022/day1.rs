use crate::register_day;

fn parse_input(input: &str) -> Vec<u32> {
    let mut cal: u32 = 0;
    let mut cal_vec: Vec<u32> = Vec::new();
    for line in input.lines() {
        if !line.is_empty() {
            cal += line.parse::<u32>().unwrap();
        } else {
            cal_vec.push(cal);
            cal = 0;
        }
    }
    // Don't forget the last group if input doesn't end with empty line
    if cal > 0 {
        cal_vec.push(cal);
    }

    cal_vec
}

fn part1(input: &str) {
    let cal_vec = parse_input(input);
    let result = cal_vec.iter().max().unwrap();
    println!("Day 1 Part 1: {result}");
}

fn part2(input: &str) {
    let mut cal_vec = parse_input(input);
    cal_vec.sort();
    let result: u32 = cal_vec.iter().rev().take(3).sum();
    println!("Day 1 Part 2: {result}");
}

register_day!(2022, 1, part1, part2);
