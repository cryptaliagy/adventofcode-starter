use adventofcode2024::*;
use clap::Parser;

#[cfg(feature = "logging")]
use tracing::{error, info};

/// Advent of Code 2024 - A Rust CLI for solving Advent of Code 2024 puzzles.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// The puzzle day to solve.
    day: u8,

    /// The part of the puzzle to solve.
    #[arg(short = 'p', long, default_value = "1")]
    part: u8,

    /// The input file to use. [default: `input/{day}.txt`]
    #[arg(short = 'f', long)]
    file: Option<String>,

    /// The verbosity level. Default is info. Set once for debug, twice or more for trace. Requires the `logging` feature.
    #[arg(short = 'v', long, action = clap::ArgAction::Count)]
    verbose: u8,
}

fn main() {
    let args = Cli::parse();

    // Setup logging based on verbosity level
    #[cfg(feature = "logging")]
    {
        let log_level = match args.verbose {
            0 => tracing::Level::INFO,
            1 => tracing::Level::DEBUG,
            _ => tracing::Level::TRACE,
        };

        let subscriber = tracing_subscriber::fmt()
            .pretty()
            .with_max_level(log_level)
            .with_target(true)
            .finish();

        tracing::subscriber::set_global_default(subscriber).unwrap();

        info!("Solving day {} part {}...", args.day, args.part);
    }
    #[cfg(not(feature = "logging"))]
    {
        println!("Solving day {} part {}...", args.day, args.part)
    }

    let input_file = args
        .file
        .clone() // Clone so there is no partial move
        .unwrap_or(format!("input/{}.txt", args.day));
    let input = match std::fs::read_to_string(input_file.as_str()) {
        Ok(input) => input,
        Err(e) => {
            #[cfg(feature = "logging")]
            {
                error!(?args, ?input_file, "Error reading input file: {}", e);
            }
            #[cfg(not(feature = "logging"))]
            {
                eprintln!("Error reading input file '{}': {}", input_file.as_str(), e);
            }
            std::process::exit(1);
        }
    };

    // Start the timer only if opting into timing through a feature flag.
    #[cfg(feature = "metrics")]
    let start = std::time::SystemTime::now();

    let result = match args.day {
        1 => match args.part {
            1 => day_one::part_one(&input),
            2 => day_one::part_two(&input),
            _ => {
                #[cfg(feature = "logging")]
                {
                    error!(?args, "Invalid part number",);
                }
                #[cfg(not(feature = "logging"))]
                {
                    eprintln!("Invalid part number: {}", args.part);
                }
                std::process::exit(1);
            }
        },
        // Add more solutions here below!
        // 2 => match args.part {
        //     1 => day_two::part_one(&input),
        //     2 => day_two::part_two(&input),
        //     _ => {
        //         #[cfg(feature = "logging")]
        //         {
        //             error!(?args, "Invalid part number",);
        //         }
        //         #[cfg(not(feature = "logging"))]
        //         {
        //             eprintln!("Invalid part number: {}", args.part);
        //         }
        //         std::process::exit(1);
        //     }
        // },
        _ => {
            #[cfg(feature = "logging")]
            {
                error!(?args, "Invalid day number");
            }
            #[cfg(not(feature = "logging"))]
            {
                eprintln!("Invalid day number: {}", args.day);
            }
            std::process::exit(1);
        }
    };

    #[cfg(feature = "metrics")]
    let elapsed = start.elapsed().unwrap();

    #[cfg(feature = "logging")]
    {
        info!("Result: {}", result);
    }
    #[cfg(not(feature = "logging"))]
    {
        println!("Result: {}", result);
    }

    #[cfg(feature = "metrics")]
    {
        #[cfg(feature = "logging")]
        {
            info!("Solution found in {}", utils::format_duration(elapsed));
        }
        #[cfg(not(feature = "logging"))]
        {
            println!("Solution found in {}", utils::format_duration(elapsed));
        }
    }
}
