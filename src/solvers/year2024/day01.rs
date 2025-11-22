use nom::{
    IResult, Parser,
    character::complete::{i32 as nom_i32, line_ending, multispace1},
    combinator::all_consuming,
    multi::separated_list1,
    sequence::separated_pair,
};

use std::iter::zip;

fn parse_pair(input: &str) -> IResult<&str, (i32, i32)> {
    separated_pair(nom_i32, multispace1, nom_i32).parse(input)
}

fn parse(input: &str) -> IResult<&str, (Vec<i32>, Vec<i32>)> {
    // remove trailing whitespace
    let input = input.trim();
    let (input, pairs) = all_consuming(separated_list1(line_ending, parse_pair)).parse(input)?;
    Ok((input, pairs.into_iter().unzip()))
}

pub fn part1(input: &str) -> Result<String, String> {
    let (_, (mut left, mut right)) = parse(input).map_err(|e| e.to_string())?;

    left.sort();
    right.sort();

    let distance: i32 = zip(left, right).map(|p| (p.0 - p.1).abs()).sum();

    Ok(distance.to_string())
}

pub fn part2(input: &str) -> Result<String, String> {
    let (_, (left, right)) = parse(input).map_err(|e| e.to_string())?;

    // could use a hashmap here for efficiency
    let similarity: i32 = left
        .iter()
        .map(|&v| v * right.iter().filter(|&rv| *rv == v).count() as i32)
        .sum();

    Ok(similarity.to_string())
}
