use crate::register_day;
use hashbrown::{HashMap, HashSet};
use itertools::Itertools;
use std::{
    cmp::Ordering,
    i64::{MAX, MIN},
    ops::RangeInclusive,
};

type Sensor = Loc;
type Beacon = Loc;

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
struct Loc {
    x: i64,
    y: i64,
}

impl Loc {
    fn manhattan(&self, other: &Loc) -> i64 {
        self.x_dist(other.x) + self.y_dist(other.y)
    }

    fn x_dist(&self, other_x: i64) -> i64 {
        (self.x - other_x).abs()
    }

    fn y_dist(&self, other_y: i64) -> i64 {
        (self.y - other_y).abs()
    }
}

/// A function that takes two inclusive ranges, and joins them if they overlap.
fn range_join(a: &RangeInclusive<i64>, b: &RangeInclusive<i64>) -> Option<RangeInclusive<i64>> {
    match (a.start().cmp(b.end()), a.end().cmp(b.start())) {
        (Ordering::Greater, _) => None,
        (_, Ordering::Less) => None,
        (_, _) => Some(*a.start().min(b.start())..=*a.end().max(b.end())),
    }
}

fn parse_input(input: &str) -> (HashMap<Sensor, i64>, HashSet<Beacon>) {
    let mut beacons: HashSet<Beacon> = HashSet::new();
    let sensors = input
        .lines()
        .filter_map(|line| {
            let parts: Vec<i64> = line
                .split(|c: char| !c.is_ascii_digit() && c != '-')
                .filter(|s| !s.is_empty())
                .filter_map(|s| s.parse().ok())
                .collect();
            if parts.len() >= 4 {
                let s = Loc {
                    x: parts[0],
                    y: parts[1],
                };
                let b = Loc {
                    x: parts[2],
                    y: parts[3],
                };
                let d = s.manhattan(&b);
                beacons.insert(b);
                Some((s, d))
            } else {
                None
            }
        })
        .collect::<HashMap<_, _>>();

    (sensors, beacons)
}

fn compress_ranges(ranges: &mut Vec<RangeInclusive<i64>>) {
    loop {
        if ranges.len() <= 1 {
            break;
        }

        let mut altered = false;
        for _ in 0..ranges.len() {
            let a = ranges.remove(0);
            let rc = ranges.clone();
            let f = rc.iter().find_position(|b| range_join(&a, b).is_some());

            match f {
                Some((u, b)) => {
                    ranges.remove(u);
                    ranges.push(range_join(&a, b).unwrap());
                    altered = true;
                    break;
                }
                None => {
                    ranges.push(a);
                }
            }
        }

        if !altered {
            break;
        }
    }
}

fn determine_ranges(
    sensors: &HashMap<Sensor, i64>,
    target_row: i64,
    mn: i64,
    mx: i64,
) -> Vec<RangeInclusive<i64>> {
    sensors
        .iter()
        .filter_map(|(s, d)| {
            let offset = d - s.y_dist(target_row);
            if offset > 0 {
                Some((s.x - offset).max(mn)..=(s.x + offset).min(mx))
            } else {
                None
            }
        })
        .collect()
}

fn part1(input: &str) {
    // Detect if using example (small numbers) or real input
    let (sensors, beacons) = parse_input(input);
    let target_row = if sensors.len() <= 14 { 10 } else { 2_000_000 };

    let mut ranges = determine_ranges(&sensors, target_row, MIN, MAX);
    compress_ranges(&mut ranges);

    let mut res = ranges
        .iter()
        .map(|r| (r.end() - r.start()).abs() + 1)
        .sum::<i64>();

    res -= beacons.iter().filter(|&&b| b.y == target_row).count() as i64;

    println!("Day 15 Part 1: {res}");
}

fn part2(input: &str) {
    let (sensors, _) = parse_input(input);
    let (t_min, t_max) = (0, if sensors.len() <= 14 { 20 } else { 4_000_000 });

    let (y, _, r) = (t_min..=t_max)
        .map(|target_row| {
            let mut ranges = determine_ranges(&sensors, target_row, t_min, t_max);
            compress_ranges(&mut ranges);
            (target_row, ranges)
        })
        .map(|(row, ranges)| {
            let size = ranges
                .iter()
                .map(|r| (r.end() - r.start()).abs() + 1)
                .sum::<i64>();

            (row, size - 1, ranges)
        })
        .find(|(_, s, _)| *s < t_max)
        .unwrap();

    let x = (t_min..=t_max)
        .find(|x| !r.iter().any(|r| r.contains(x)))
        .unwrap();

    let res = x * 4_000_000 + y;

    println!("Day 15 Part 2: {res} (x={x}, y={y})");
}

register_day!(2022, 15, part1, part2);
