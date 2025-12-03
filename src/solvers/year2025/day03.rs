use nom::{
    IResult, Parser,
    character::complete::line_ending,
    combinator::all_consuming,
    multi::{many1, separated_list1},
};

use crate::utils::nom_ext::one_digit;

fn parse_bank(input: &str) -> IResult<&str, Vec<i64>> {
    many1(one_digit).parse(input)
}

fn parse(input: &str) -> Result<Vec<Vec<i64>>, String> {
    let (_, banks) = all_consuming(separated_list1(line_ending, parse_bank))
        .parse(input.trim())
        .map_err(|e| e.to_string())?;
    Ok(banks)
}

fn joltage<const N: usize>(bank: Vec<i64>) -> i64 {
    let mut candidates: Vec<i64> = bank.iter().rev().take(N).rev().copied().collect();
    for value in bank.into_iter().rev().skip(N) {
        let mut value = value;
        for c in &mut candidates {
            if *c <= value {
                std::mem::swap(c, &mut value);
            } else {
                break;
            }
        }
    }
    candidates.iter().fold(0, |acc, x| acc * 10 + x)
}

pub fn part1(input: &str) -> Result<String, String> {
    let banks = parse(input)?;
    let total_joltage: i64 = banks.into_iter().map(joltage::<2>).sum();
    Ok(total_joltage.to_string())
}

pub fn part2(input: &str) -> Result<String, String> {
    let banks = parse(input)?;
    let total_joltage: i64 = banks.into_iter().map(joltage::<12>).sum();
    Ok(total_joltage.to_string())
}
