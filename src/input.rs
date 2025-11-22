use std::error::Error;
use std::fs;
use std::path::Path;

use crate::model::Input;

fn read_to_string(year: &str, day: &str, file: &str) -> std::io::Result<String> {
    fs::read_to_string(
        Path::new(".")
            .join("inputs")
            .join(year)
            .join(day)
            .join(file),
    )
}

/// Reads the example files for the given year. If part1_ex.txt does not exist, this returns an error.
pub fn get_example(year: &str, day: &str) -> Result<Input, Box<dyn Error>> {
    let part1 = read_to_string(year, day, "part1_ex.txt")?;
    let part2 = read_to_string(year, day, "part2_ex.txt").ok();

    let solution1 = read_to_string(year, day, "solution1_ex.txt").ok();
    let solution2 = read_to_string(year, day, "solution2_ex.txt").ok();

    Ok(Input {
        part1,
        part2,
        solution1,
        solution2,
    })
}

/// Reads the example files for the given year. If part1.txt does not exist, this returns an error.
pub fn get_problem(year: &str, day: &str) -> Result<Input, Box<dyn Error>> {
    let part1 = read_to_string(year, day, "part1.txt")?;
    let part2 = read_to_string(year, day, "part2.txt").ok();

    let solution1 = read_to_string(year, day, "solution1.txt").ok();
    let solution2 = read_to_string(year, day, "solution2.txt").ok();

    Ok(Input {
        part1,
        part2,
        solution1,
        solution2,
    })
}
