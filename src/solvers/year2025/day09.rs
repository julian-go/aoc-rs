use std::collections::{HashMap, HashSet};

use nom::{
    IResult, Parser,
    character::complete::{char, i64 as nom_i64, line_ending},
    multi::separated_list1,
    sequence::separated_pair,
};

fn parse(input: &str) -> IResult<&str, Vec<(i64, i64)>> {
    separated_list1(line_ending, separated_pair(nom_i64, char(','), nom_i64)).parse(input)
}

fn rect_size(a: (i64, i64), b: (i64, i64)) -> i64 {
    let width = (a.0 - b.0).abs() + 1;
    let height = (a.1 - b.1).abs() + 1;

    width * height
}

pub fn part1(input: &str) -> Result<String, String> {
    let (_, tiles) = parse(input).map_err(|e| e.to_string())?;

    let mut max_rectangle: i64 = 0;
    for i in 0..tiles.len() {
        for j in i..tiles.len() {
            max_rectangle = max_rectangle.max(rect_size(tiles[i], tiles[j]));
        }
    }

    Ok(max_rectangle.to_string())
}

pub fn part2(input: &str) -> Result<String, String> {
    Ok(1.to_string())
}
