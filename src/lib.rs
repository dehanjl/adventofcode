use clap::Parser;
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::LazyLock;
use std::time::Instant;
use std::{env, fs};

use anyhow::Result;
use reqwest::header::COOKIE;

pub mod solutions;
pub mod utils;

/// Function type for solution parts
pub type SolutionFn = fn(&str);

/// A registered day solution
pub struct DaySolution {
    pub year: u16,
    pub day: u8,
    pub part1: SolutionFn,
    pub part2: Option<SolutionFn>,
}

// Allow solutions to register themselves
inventory::collect!(DaySolution);

/// Lookup table built from all registered solutions
pub static SOLUTIONS: LazyLock<HashMap<(u16, u8), &'static DaySolution>> = LazyLock::new(|| {
    inventory::iter::<DaySolution>
        .into_iter()
        .map(|s| ((s.year, s.day), s))
        .collect()
});

/// Get a solution by year and day
pub fn get_solution(year: u16, day: u8) -> Option<&'static DaySolution> {
    SOLUTIONS.get(&(year, day)).copied()
}

/// List all registered solutions
pub fn list_solutions() -> Vec<(u16, u8)> {
    let mut keys: Vec<_> = SOLUTIONS.keys().copied().collect();
    keys.sort();
    keys
}

/// Macro to register a day solution
#[macro_export]
macro_rules! register_day {
    // With both parts
    ($year:expr, $day:expr, $part1:expr, $part2:expr) => {
        inventory::submit! {
            $crate::DaySolution {
                year: $year,
                day: $day,
                part1: $part1,
                part2: Some($part2),
            }
        }
    };
    // With only part 1
    ($year:expr, $day:expr, $part1:expr) => {
        inventory::submit! {
            $crate::DaySolution {
                year: $year,
                day: $day,
                part1: $part1,
                part2: None,
            }
        }
    };
}

#[derive(Parser)]
#[command(name = "aoc")]
#[command(about = "Advent of Code runner")]
pub struct Opt {
    /// Year (e.g., 25 or 2025). Defaults to current year.
    #[arg(short, long, default_value_t = 2025)]
    pub year: u16,

    /// Day (1-25)
    #[arg(short, long)]
    pub day: Option<u8>,

    /// Which part to run (1, 2, or both if omitted)
    #[arg(short, long)]
    pub part: Option<u8>,

    /// Use real input instead of example
    #[arg(short, long)]
    pub real: bool,

    /// Alternative input file name (without extension)
    #[arg(short, long)]
    pub alt: Option<String>,

    /// AOC session token (overrides .env and environment variable)
    #[arg(long)]
    pub session: Option<String>,

    /// List all available solutions
    #[arg(long)]
    pub list: bool,
}

impl Opt {
    /// Parse CLI arguments
    pub fn get() -> Self {
        // Load .env file if it exists (before parsing args)
        let _ = dotenvy::dotenv();
        Opt::parse()
    }

    /// Normalize year (25 -> 2025)
    pub fn normalized_year(&self) -> u16 {
        if self.year < 100 {
            2000 + self.year
        } else {
            self.year
        }
    }
}

/// Run a solution function with timing
pub fn runner(f: SolutionFn, input: &str) {
    println!("---");
    let start = Instant::now();
    f(input);
    let duration = start.elapsed();
    println!("--- {duration:?}")
}

/// Get input for a specific year/day
pub fn get_input_for_day(opt: &Opt, year: u16, day: u8) -> Result<String> {
    let path = make_path(year, day, opt);

    match (path.exists(), opt.real) {
        (true, _) => fs::read_to_string(path).map_err(anyhow::Error::from),
        (false, false) => anyhow::bail!("Example input not found: {}", path.display()),
        (false, true) => download_and_save(opt, path, year, day),
    }
}

fn make_path(year: u16, day: u8, opt: &Opt) -> PathBuf {
    let filename = format!("day{day}");
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));

    path.push("inputs");
    path.push(format!("y{year}"));
    path.push(if opt.real { "real" } else { "example" });
    path.push(if opt.real || opt.alt.is_none() {
        &filename
    } else {
        opt.alt.as_ref().unwrap()
    });
    path.set_extension("txt");

    path
}

fn download_and_save(opt: &Opt, path: PathBuf, year: u16, day: u8) -> Result<String> {
    // Create parent directories if needed
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let resp = download_input(opt, year, day)?;
    fs::write(&path, resp.as_bytes())?;
    Ok(resp)
}

fn download_input(opt: &Opt, year: u16, day: u8) -> Result<String> {
    let client = reqwest::blocking::Client::new();
    let resp = client
        .get(make_url(year, day))
        .header(
            COOKIE,
            String::from("session=") + get_session_token(opt)?.as_str(),
        )
        .send()?
        .text()?;

    Ok(resp)
}

fn make_url(year: u16, day: u8) -> String {
    format!("https://adventofcode.com/{year}/day/{day}/input")
}

/// Get session token with priority: CLI arg > .env > env var
fn get_session_token(opt: &Opt) -> Result<String> {
    // 1. Check CLI argument
    if let Some(session) = &opt.session {
        return Ok(session.clone());
    }

    // 2. Check environment variable (dotenvy already loaded .env)
    env::var("AOC_SESSION").map_err(|_| {
        anyhow::anyhow!(
            "AOC_SESSION not found. Set it via:\n\
             - CLI: --session <token>\n\
             - .env file: AOC_SESSION=<token>\n\
             - Environment variable: export AOC_SESSION=<token>"
        )
    })
}
