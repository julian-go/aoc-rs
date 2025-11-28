mod input;
mod model;
mod solvers;
mod utils;

use std::time::Instant;

use clap::Parser;
use comfy_table::{Cell, Color, Table};

use model::Outcome;

/// Solver for advent of code written in rust 🦀
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// The year to run
    #[arg(short, long)]
    year: Option<u32>,

    /// The day to run, only used with --year
    #[arg(short, long)]
    day: Option<u32>,

    /// Run only the last day of the latest year
    #[arg(short, long, name = "last")]
    last_only: bool,

    /// Run only the examples
    #[arg(short, long, name = "examples")]
    examples_only: bool,
}

fn main() {
    let args = Args::parse();

    let mut results: Vec<Outcome> = vec![];

    if args.last_only {
        run_last(args.examples_only, &mut results);
    } else if let Some(year) = args.year.map(|y| y.to_string()) {
        if let Some(day) = args.day.map(|d| format!("day{d:02}")) {
            run_day_in_year(&year, &day, args.examples_only, &mut results);
        } else {
            run_year(&year, args.examples_only, &mut results);
        }
    } else {
        run_all(args.examples_only, &mut results);
    }

    if results.is_empty() {
        println!("No days were run.");
        return;
    }

    let mut table = Table::new();
    table.set_header(vec![
        "year",
        "day",
        "part",
        "result",
        "solution",
        "correct",
        "elapsed ms",
    ]);
    for outcome in &results {
        table.add_row(vec![
            Cell::new(outcome.year),
            Cell::new(outcome.day),
            Cell::new(outcome.part.as_str()),
            Cell::new(outcome.result.as_str()),
            Cell::new(outcome.solution.as_str()),
            if outcome.correct {
                Cell::new("yes").fg(Color::Green)
            } else {
                Cell::new("no").fg(Color::Red)
            },
            Cell::new(outcome.elapsed_ms.to_string()),
        ]);
    }
    println!("{table}");
}

fn run_year(year_arg: &str, example_only: bool, results: &mut Vec<Outcome>) {
    for (year, days) in solvers::YEARS {
        if year_arg == *year {
            for day in *days {
                run_day(year, day, true, results);
                if !example_only {
                    run_day(year, day, false, results);
                }
            }
        }
    }
}

fn run_day_in_year(year_arg: &str, day_arg: &str, example_only: bool, results: &mut Vec<Outcome>) {
    for (year, days) in solvers::YEARS {
        if year_arg == *year {
            for day in *days {
                if day_arg == day.name {
                    run_day(year, day, true, results);
                    if !example_only {
                        run_day(year, day, false, results);
                    }
                }
            }
        }
    }
}

fn run_last(example_only: bool, results: &mut Vec<Outcome>) {
    if let Some(year) = solvers::YEARS.last() {
        if let Some(day) = year.1.last() {
            run_day(year.0, day, true, results);
            if !example_only {
                run_day(year.0, day, false, results);
            }
        }
    }
}

fn run_all(example_only: bool, results: &mut Vec<Outcome>) {
    for (year, days) in solvers::YEARS {
        for day in *days {
            run_day(year, day, true, results);
            if !example_only {
                run_day(year, day, false, results);
            }
        }
    }
}

fn run_day(year: &'static str, day: &solvers::Day, example: bool, results: &mut Vec<Outcome>) {
    let Ok(input) = (if example {
        input::get_example(year, day.name)
    } else {
        input::get_problem(year, day.name)
    }) else {
        if example {
            eprintln!("Failed to get example input for {year} {}", day.name);
        } else {
            eprintln!("Failed to get input for {year} {}", day.name);
        }
        return;
    };

    if let Some((result, correct, elapsed_ms)) =
        run_part(&input.part1, day.part1, input.solution1.as_deref())
    {
        results.push(Outcome {
            year,
            // unwrapping here is safe because day names always start with "day"
            day: day.name.strip_prefix("day").unwrap(),
            part: if example { "1ex" } else { "1" }.to_string(),
            result,
            solution: input.solution1.unwrap_or_default(),
            correct,
            elapsed_ms,
        });
    }

    if let Some((result, correct, elapsed_ms)) = run_part(
        input.part2.as_deref().unwrap_or(&input.part1),
        day.part2,
        input.solution2.as_deref(),
    ) {
        results.push(Outcome {
            year,
            day: day.name.strip_prefix("day").unwrap(),
            part: if example { "2ex" } else { "2" }.to_string(),
            result,
            solution: input.solution2.unwrap_or_default(),
            correct,
            elapsed_ms,
        });
    }
}

fn run_part(
    input: &str,
    solver: solvers::SolveFn,
    solution: Option<&str>,
) -> Option<(String, bool, i32)> {
    let now = Instant::now();
    match solver(input) {
        Ok(result) => {
            let correct = solution.is_some_and(|expected| expected == result);
            Some((
                result,
                correct,
                now.elapsed().as_millis().try_into().unwrap(),
            ))
        }
        Err(e) => {
            eprintln!("{e}");
            None
        }
    }
}
