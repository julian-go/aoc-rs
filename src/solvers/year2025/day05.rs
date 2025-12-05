use nom::{
    IResult, Parser,
    character::complete::{char, i64 as nom_i64, line_ending, one_of},
    combinator::{all_consuming, map},
    multi::separated_list1,
    sequence::separated_pair,
};

use crate::utils::Interval;

pub fn parse_is_roll(input: &str) -> IResult<&str, bool> {
    map(one_of("@."), |c| c == '@').parse(input)
}

fn parse_interval(input: &str) -> IResult<&str, Interval> {
    map(separated_pair(nom_i64, char('-'), nom_i64), |(a, b)| {
        Interval::new(a, b + 1)
    })
    .parse(input)
}

fn parse_ingredients(input: &str) -> IResult<&str, Vec<i64>> {
    separated_list1(line_ending, nom_i64).parse(input)
}

pub fn parse(input: &str) -> Result<(Vec<Interval>, Vec<i64>), String> {
    let (input, intervals) = separated_list1(line_ending, parse_interval)
        .parse(input)
        .map_err(|e| e.to_string())?;

    let (_, ingredients) = all_consuming(parse_ingredients)
        .parse(input.trim())
        .map_err(|e| e.to_string())?;

    Ok((intervals, ingredients))
}

pub fn part1(input: &str) -> Result<String, String> {
    let (ranges, ingredients) = parse(input)?;

    let fresh_count = ingredients
        .into_iter()
        .filter(move |ingredient| ranges.iter().any(|range| range.includes(*ingredient)))
        .count();

    Ok(fresh_count.to_string())
}

pub fn part2(input: &str) -> Result<String, String> {
    let (mut ranges, _) = parse(input)?;

    'outer: loop {
        for i in 0..ranges.len() {
            for j in (i + 1)..ranges.len() {
                if ranges[i].overlaps(ranges[j]) {
                    ranges[i] = ranges[i].union(ranges[j]);
                    ranges.swap_remove(j);
                    continue 'outer;
                }
            }
        }

        break;
    }

    let total_fresh: usize = ranges.into_iter().map(|range| range.len()).sum();

    Ok(total_fresh.to_string())
}
