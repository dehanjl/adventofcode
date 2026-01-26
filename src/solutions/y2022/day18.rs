use crate::register_day;
use hashbrown::HashSet;
use itertools::Itertools;

type Loc = (i32, i32, i32);

fn sides((x, y, z): Loc) -> [Loc; 6] {
    [
        (x - 1, y, z),
        (x + 1, y, z),
        (x, y - 1, z),
        (x, y + 1, z),
        (x, y, z - 1),
        (x, y, z + 1),
    ]
}

fn parse_input(input: &str) -> HashSet<Loc> {
    input
        .lines()
        .filter_map(|line| {
            let parts: Vec<i32> = line.split(',').filter_map(|s| s.parse().ok()).collect();
            if parts.len() == 3 {
                Some((parts[0], parts[1], parts[2]))
            } else {
                None
            }
        })
        .collect()
}

fn part1(input: &str) {
    let drops = parse_input(input);

    let surface_area = drops
        .iter()
        .flat_map(|&p| sides(p))
        .filter(|s| !drops.contains(s))
        .count();

    println!("Day 18 Part 1: {surface_area}");
}

fn part2(input: &str) {
    let drops = parse_input(input);
    let max = *drops.iter().flat_map(|(x, y, z)| [x, y, z]).max().unwrap();

    let mut steam: HashSet<Loc> = HashSet::new();
    let mut stack = vec![(0, 0, 0)];
    while let Some(loc) = stack.pop() {
        let new_locs = sides(loc)
            .iter()
            .filter(|&s| !drops.contains(s))
            .filter(|&s| !steam.contains(s))
            .filter(|&&(x, y, z)| [x, y, z].iter().all(|&i| -1 <= i && i <= max + 1))
            .copied()
            .collect_vec();

        steam.extend(new_locs.iter());
        stack.extend(new_locs.iter());
    }

    let surface_area = drops
        .iter()
        .flat_map(|&p| sides(p))
        .filter(|s| steam.contains(s))
        .count();

    println!("Day 18 Part 2: {surface_area}");
}

register_day!(2022, 18, part1, part2);
