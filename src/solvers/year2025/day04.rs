use nom::{
    IResult, Parser,
    character::complete::{line_ending, one_of},
    combinator::{all_consuming, map, map_res},
    multi::{many1, separated_list1},
};

use crate::utils::{Matrix, Vector};

pub fn parse_is_roll(input: &str) -> IResult<&str, bool> {
    map(one_of("@."), |c| c == '@').parse(input)
}

pub fn parse(input: &str) -> Result<Matrix<bool>, String> {
    let (_, map) = all_consuming(map_res(
        separated_list1(line_ending, many1(parse_is_roll)),
        |lines| {
            let width = lines.first().map_or(0, std::vec::Vec::len);
            let values = lines.into_iter().flatten().collect();
            Matrix::from_vec(width, values)
        },
    ))
    .parse(input.trim())
    .map_err(|e| e.to_string())?;
    Ok(map)
}

fn get_removable(map: &Matrix<bool>) -> Vec<Vector> {
    map.iter_positions()
        .filter(|(pos, val)| {
            *val && pos
                .adjacent()
                .iter()
                .filter_map(|&adj| map.get(adj).filter(|adj| **adj))
                .count()
                < 4
        })
        .map(|(pos, _)| pos)
        .collect()
}

pub fn part1(input: &str) -> Result<String, String> {
    let map = parse(input)?;

    let total = get_removable(&map).len();

    Ok(total.to_string())
}

pub fn part2(input: &str) -> Result<String, String> {
    let mut map = parse(input)?;

    let mut total = 0;
    loop {
        let removable = get_removable(&map);
        if removable.is_empty() {
            break;
        }
        total += removable.len();
        for pos in removable {
            if let Some(val) = map.get_mut(pos) {
                *val = false;
            }
        }
    }

    Ok(total.to_string())
}
