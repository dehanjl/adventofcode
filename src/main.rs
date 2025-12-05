use adventofcode::{Opt, SOLUTIONS, get_input_for_day, get_solution, list_solutions, runner};
use tabled::{Table, Tabled, settings::Style};

#[derive(Tabled)]
struct SolutionRow {
    #[tabled(rename = "Year")]
    year: String,
    #[tabled(rename = "Day")]
    day: u8,
    #[tabled(rename = "Parts")]
    parts: &'static str,
}

fn main() {
    let opt = Opt::get();
    let year = opt.normalized_year();

    if opt.list {
        let solutions = list_solutions();
        if solutions.is_empty() {
            println!("No solutions available yet.");
            return;
        }

        let mut rows = Vec::new();
        let mut last_year = 0;

        for (y, d) in solutions {
            let sol = SOLUTIONS.get(&(y, d)).unwrap();
            let parts = if sol.part2.is_some() { "1, 2" } else { "1" };
            let year_str = if y != last_year {
                last_year = y;
                y.to_string()
            } else {
                String::new()
            };
            rows.push(SolutionRow {
                year: year_str,
                day: d,
                parts,
            });
        }

        let table = Table::new(rows).with(Style::rounded()).to_string();
        println!("{table}");
        return;
    }

    let Some(day) = opt.day else {
        eprintln!("Please specify a day with -d <day>");
        eprintln!("Run with --list to see available solutions");
        std::process::exit(1);
    };

    let Some(solution) = get_solution(year, day) else {
        eprintln!("No solution found for {year} day {day}");
        eprintln!("Run with --list to see available solutions");
        std::process::exit(1);
    };

    let input = match get_input_for_day(&opt, year, day) {
        Ok(input) => input,
        Err(e) => {
            eprintln!("Failed to get input: {e}");
            std::process::exit(1);
        }
    };

    match opt.part {
        Some(1) => runner(solution.part1, &input),
        Some(2) => {
            if let Some(part2) = solution.part2 {
                runner(part2, &input);
            } else {
                eprintln!("Part 2 not implemented for {year} day {day}");
                std::process::exit(1);
            }
        }
        None | Some(_) => {
            runner(solution.part1, &input);
            if let Some(part2) = solution.part2 {
                runner(part2, &input);
            }
        }
    }
}
