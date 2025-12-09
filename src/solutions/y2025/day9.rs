use crate::register_day;

fn parse_input(input: &str) -> Vec<String> {
// TODO: parse the input into a more useful structure
input
.lines()
.map(|line| line.to_string())
.collect()
}

fn part1(input: &str) {
let _data = parse_input(input);
// TODO: solve part 1
println!("Day 9 Part 1: {}", "TODO");
}

fn _part2(input: &str) {
let _data = parse_input(input);
// TODO: solve part 2
println!("Day 9 Part 2: {}", "TODO");
}

register_day!(2025, 9, part1);
