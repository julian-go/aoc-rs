use crate::utils::{map::Map, nom_ext};
use nom::{
    IResult, Parser,
    character::complete::{i32 as nom_i32, line_ending, multispace1},
    combinator::{all_consuming, map_res},
    multi::{many1, separated_list1},
    sequence::separated_pair,
};

pub fn parse(input: &str) -> IResult<&str, Map<i64>> {
    all_consuming(map_res(
        separated_list1(line_ending, many1(nom_ext::one_digit)),
        |lines| {
            let width = lines.first().map(|l| l.len()).unwrap_or(0);
            let values = lines.into_iter().flatten().collect();
            Map::from_vec(width, values)
        },
    ))
    .parse(input.trim())
}

pub fn part1(input: &str) -> Result<String, String> {
    let map = parse(input).map_err(|e| e.to_string())?.1;
    println!("{map:?}");
    Ok(1.to_string())
}

pub fn part2(input: &str) -> Result<String, String> {
    let map = parse(input).map_err(|e| e.to_string())?.1;
    Ok(1.to_string())
}
