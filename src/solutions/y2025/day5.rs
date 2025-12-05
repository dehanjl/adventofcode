use crate::register_day;

struct Range {
    start: i64,
    end: i64,
}

impl Range {
    fn contains(&self, value: i64) -> bool {
        self.start <= value && value <= self.end
    }

    fn intersects(&self, other: &Range) -> bool {
        self.start <= other.end && other.start <= self.end
    }

    fn merge(&self, other: &Range) -> Range {
        Range {
            start: self.start.min(other.start),
            end: self.end.max(other.end),
        }
    }

    fn count(&self) -> i64 {
        self.end - self.start + 1
    }
}

impl From<(i64, i64)> for Range {
    fn from((start, end): (i64, i64)) -> Self {
        Self { start, end }
    }
}

fn parse_input(input: &str) -> (Vec<Range>, Vec<i64>) {
    let (ranges_str, idxs_str) = input.split_once("\n\n").unwrap();
    let ranges = ranges_str
        .lines()
        .map(|line| sscanf::sscanf!(line, "{i64}-{i64}").unwrap().into())
        .collect();
    let idxs = idxs_str.lines().map(|line| line.parse().unwrap()).collect();
    (ranges, idxs)
}

fn part1(input: &str) {
    let (ranges, idxs) = parse_input(input);
    let count = idxs
        .iter()
        .filter(|&&idx| ranges.iter().any(|range| range.contains(idx)))
        .count();
    println!("Day 5 Part 1: {}", count);
}
fn part2(input: &str) {
    let (mut ranges, _) = parse_input(input);
    ranges.sort_by_key(|r| r.start);

    let mut merged: Vec<Range> = Vec::new();
    for r in ranges {
        if let Some(last) = merged.last_mut()
            && last.intersects(&r)
        {
            *last = last.merge(&r);
            continue;
        }
        merged.push(r);
    }

    let total_count: i64 = merged.iter().map(|r| r.count()).sum();
    println!("Day 5 Part 2: {}", total_count);
}

register_day!(2025, 5, part1, part2);
