use adventofcode::{Opt, SOLUTIONS, get_input_for_day, get_solution, list_solutions, runner};
use anyhow::{Context, Result, bail};
use std::{
    fs,
    path::{Path, PathBuf},
};
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

    if opt.new {
        if let Err(e) = scaffold_day(&opt, year) {
            eprintln!("Failed to scaffold: {e}");
            std::process::exit(1);
        }
        return;
    }

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

fn scaffold_day(opt: &Opt, year: u16) -> Result<()> {
    let Some(day) = opt.day else {
        bail!("--new requires --day <day> (1-25)");
    };

    if opt.list || opt.part.is_some() || opt.real || opt.alt.is_some() {
        bail!("--new only scaffolds; remove --list, --part, --real, and --alt to continue");
    }

    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let solutions_dir = manifest_dir.join("src").join("solutions");
    let year_dir = solutions_dir.join(format!("y{year}"));
    fs::create_dir_all(&year_dir).context("creating year solutions directory")?;

    ensure_year_mod(&solutions_dir, year)?;

    let day_path = year_dir.join(format!("day{day}.rs"));
    if day_path.exists() {
        bail!(
            "Day {day} for {year} already exists at {}",
            day_path.display()
        );
    }

    let template = make_day_template(year, day);
    fs::write(&day_path, template).with_context(|| format!("writing {}", day_path.display()))?;

    println!("Created {}", day_path.display());
    println!(
        "Next: add inputs under inputs/y{year}/example/day{day}.txt and run with: cargo run -- --year {year} --day {day}",
    );

    Ok(())
}

fn ensure_year_mod(solutions_dir: &Path, year: u16) -> Result<()> {
    let year_dir = solutions_dir.join(format!("y{year}"));
    let year_mod = year_dir.join("mod.rs");

    if !year_mod.exists() {
        fs::write(
            &year_mod,
            format!("automod::dir!(\"src/solutions/y{year}\");\n"),
        )
        .with_context(|| format!("writing {}", year_mod.display()))?;
    }

    ensure_solutions_mod(solutions_dir, year)
}

fn ensure_solutions_mod(solutions_dir: &Path, year: u16) -> Result<()> {
    let solutions_mod = solutions_dir.join("mod.rs");
    let mut contents = fs::read_to_string(&solutions_mod)
        .with_context(|| format!("reading {}", solutions_mod.display()))?;

    let line = format!("pub mod y{year};");
    let has_line = contents.lines().any(|l| l.trim() == line);
    if !has_line {
        if !contents.ends_with('\n') {
            contents.push('\n');
        }
        contents.push_str(&line);
        contents.push('\n');
        fs::write(&solutions_mod, contents)
            .with_context(|| format!("updating {}", solutions_mod.display()))?;
    }

    Ok(())
}

fn make_day_template(year: u16, day: u8) -> String {
    format!(
        "use crate::register_day;\n\
         \n\
         fn parse_input(input: &str) -> Vec<String> {{\n\
             // TODO: parse the input into a more useful structure\n\
             input\n\
                 .lines()\n\
                 .map(|line| line.to_string())\n\
                 .collect()\n\
         }}\n\
         \n\
         fn part1(input: &str) {{\n\
             let _data = parse_input(input);\n\
             // TODO: solve part 1\n\
             println!(\"Day {day} Part 1: {{}}\", \"TODO\");\n\
         }}\n\
         \n\
         fn part2(input: &str) {{\n\
             let _data = parse_input(input);\n\
             // TODO: solve part 2\n\
             println!(\"Day {day} Part 2: {{}}\", \"TODO\");\n\
         }}\n\
         \n\
         register_day!({year}, {day}, part1, part2);\n",
        year = year,
        day = day
    )
}
