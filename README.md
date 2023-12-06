# Advent of Code Starter Project - Rust

This repository contains a starter Rust project to use with Advent of Code. This has none of the solutions to the puzzles, but includes some basic bootstrapping (i.e. reading the input file) and sets up a way to select the puzzle to execute.

There is also a basic shell for Day 1 to show how the interface works.

## Getting started

1. Make sure you have [Rust](https://www.rust-lang.org/) installed and set up with your IDE. I'm a big fan of [Visual Studio Code](https://code.visualstudio.com/) for Rust, and use the [Rust extension](https://code.visualstudio.com/docs/languages/rust), but you can check out [other tools](https://www.rust-lang.org/tools) for your favourite IDE.
1. If you're just starting out with Rust, I recommend checking out the [Rust book](https://doc.rust-lang.org/book/), but there are other Rust-based resources out there that might be more your style! Another popular place to get support is [r/LearnRust](https://www.reddit.com/r/learnrust/), and there are books from big publishers like O'Reilly, and videos from platforms like PluralSight that can help
1. If you would like to use external packages (which Rust calls 'crates'), you can check out [crates.io](https://crates.io/) to search, and [docs.rs](https://docs.rs/) for documentation. For most of the 'standard' things, there is also a community-maintained list of recommended packages available at [blessed.rs](https://blessed.rs/crates)
1. Check out the [Advent of Code day 1](https://adventofcode.com/2023/day/1) problem, and start writing your solution in the [day one source file](src/day_one.rs)
1. Update the [LICENSE](/LICENSE) file with your name

## Getting your results

1. Add a call to your solution in `src/main.rs`
1. Run `cargo run -- <day>` for part 1, `cargo run -- <day> -p 2` for part 2. If you'd like to select your input file, you can also do so with `-f <file>`
1. Your output should be shown as the last line of text!

Run `cargo run -- --help` from the command line to see the documentation for the CLI.

## What's included in this?

1. A `main.rs` binary including:
   - Basic CLI parsing (pick puzzle, puzzle part, and input file location) with [clap](https://docs.rs/clap/latest/clap/)
   - Input file reading and passing to your solution
   - Basic logging setup (with [tracing](https://docs.rs/tracing/latest/tracing/)) - OPTIONAL
   - Basic metrics setup - OPTIONAL
   - Puzzle solution output
1. A `lib.rs` that references the `day_one` module.
1. A bare-bones `day_one.rs`.
1. A basic Github Actions workflow file for running automated tests

## Opting out of logging/timing setup

If you do not want to use the included logging setup or the solution metrics, you can disable these by disabling the relevant feature flags in `Cargo.toml`. To do so, remove the feature flags you do not want to use from the `defaults` line, under `features`, like so:

```toml
[features]
default = []
logging = ["tracing", "tracing-subscriber"]
metrics = []
```

This will prevent the optional dependencies from being compiled, and will not enable the additional functionality associated with these features.
