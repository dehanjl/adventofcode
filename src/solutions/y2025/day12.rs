use crate::register_day;

type ShapeArea = u64;
struct Region {
    area: u64,
    counts: Vec<u64>,
}

fn parse_input(input: &str) -> (Vec<ShapeArea>, Vec<Region>) {
    let sections = input.split("\n\n");
    let shapes = sections
        .clone()
        .take(6)
        .map(|s| s.chars().filter(|&c| c == '#').count() as u64)
        .collect();

    let regions = sections
        .skip(6)
        .flat_map(|section| section.lines())
        .map(|line| {
            let (width, length, a, b, c, d, e, f) =
                sscanf::sscanf!(line, "{u64}x{u64}: {u64} {u64} {u64} {u64} {u64} {u64}").unwrap();
            Region {
                area: width * length,
                counts: vec![a, b, c, d, e, f],
            }
        })
        .collect();

    return (shapes, regions);
}

fn dot_prod(a: &Vec<u64>, b: &Vec<u64>) -> u64 {
    a.iter().zip(b.iter()).map(|(x, y)| x * y).sum()
}

fn part1(input: &str) {
    let (shapes, regions) = parse_input(input);

    let (mut no, mut maybe, mut yes) = (0u64, 0u64, 0u64);

    for region in regions {
        let min_area = dot_prod(&shapes, &region.counts);
        let max_area = region.counts.iter().sum::<u64>() * 9;
        if min_area > region.area {
            no += 1;
        } else if max_area < region.area {
            yes += 1;
        } else {
            maybe += 1;
        }
    }

    println!(
        "Day 12 Part 1: no={}, maybe={}, yes={}, maybe+yes={}",
        no,
        maybe,
        yes,
        maybe + yes
    );
}

register_day!(2025, 12, part1);
