# Advent of Code
A (mostly) Rust 🦀 set of solutions to the [Advent of Code](https://adventofcode.com/) puzzles.

## Usage
```bash
cargo run -- -d <day>                # Run with example input (current year)
cargo run -- -y <year> -d <day>      # Run specific year/day
cargo run -- -d <day> -r             # Run with real input
cargo run -- -d <day> -p 1           # Run only part 1
cargo run -- --list                  # List all available solutions
cargo run -- --help                  # Show all options
```

The year defaults to 2025 and accepts both short (25) and full (2025) format.

## Session Token
The runner expects example input to be provided. It will automatically download real input when needed.

Set your AOC session token using one of these methods (in priority order):
```bash
# Option 1: Command line argument
cargo run -- -d 1 -r --session <token>

# Option 2: .env file (create from .env.example)
echo "AOC_SESSION=<token>" > .env

# Option 3: Environment variable
export AOC_SESSION=<token>
```

## Folder Structure
```
.
├── inputs
│   └── y<year>
│       ├── example     # example puzzle inputs
│       │   └── dayX.txt
│       └── real        # real puzzle inputs (auto-downloaded)
│           └── dayX.txt
└── src
    ├── solutions
    │   └── y<year>
    │       ├── mod.rs  # automod for day discovery
    │       └── dayX.rs # solution for day X
    ├── lib.rs          # runner, CLI, utilities
    ├── main.rs         # main entry point
    └── utils.rs        # grid/direction helpers
```

## Adding a New Solution
1. Create `src/solutions/y<year>/day<X>.rs`
2. Add the registration macro at the bottom:
   ```rust
   use crate::register_day;
   
   fn part1(input: &str) { /* ... */ }
   fn part2(input: &str) { /* ... */ }
   
   register_day!(<year>, <day>, part1, part2);
   ```
3. For a new year, create `src/solutions/y<year>/mod.rs` with:
   ```rust
   automod::dir!("src/solutions/y<year>");
   ```
   And add `pub mod y<year>;` to `src/solutions/mod.rs`

## Helpful Resources
- A wonderful [series of articles](https://fasterthanli.me/series/advent-of-code-2022) explaining Rust 🦀 using Advent of Code 2022 by [@fasterthanlime](https://github.com/fasterthanlime)
