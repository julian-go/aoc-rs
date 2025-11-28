use std::collections::{HashSet, VecDeque};

use nom::{
    IResult, Parser,
    character::complete::line_ending,
    combinator::{all_consuming, map_res},
    multi::{many1, separated_list1},
};

use crate::utils::{Map, Pose, nom_ext};

pub fn parse(input: &str) -> IResult<&str, Map<i64>> {
    all_consuming(map_res(
        separated_list1(line_ending, many1(nom_ext::one_digit)),
        |lines| {
            let width = lines.first().map_or(0, std::vec::Vec::len);
            let values = lines.into_iter().flatten().collect();
            Map::from_vec(width, values)
        },
    ))
    .parse(input.trim())
}

fn trailhead_score(map: &Map<i64>, pos: Pose) -> i64 {
    let mut queue = VecDeque::new();
    let mut reachable = HashSet::new();
    queue.push_back(pos);
    while let Some(current_pos) = queue.pop_front() {
        let Some(&current_height) = map.get(current_pos) else {
            continue;
        };

        current_pos
            .adjacent()
            .iter()
            .filter_map(|&adjacent_pos| map.get(adjacent_pos).map(|&height| (adjacent_pos, height)))
            .filter(|(_, height)| *height == current_height + 1)
            .for_each(|(adjacent_pos, height)| {
                if height == 9 {
                    reachable.insert(adjacent_pos);
                } else {
                    queue.push_back(adjacent_pos);
                }
            });
    }
    reachable.len() as i64
}

pub fn part1(input: &str) -> Result<String, String> {
    let map = parse(input).map_err(|e| e.to_string())?.1;

    let mut scores = Vec::new();
    for y in 0..map.height() {
        for x in 0..map.width() {
            let pos = Pose::new_xy(x as i64, y as i64);
            if map.get(pos).is_some_and(|v| *v == 0) {
                scores.push(trailhead_score(&map, pos));
            }
        }
    }
    Ok(scores.iter().sum::<i64>().to_string())
}

fn trailhead_score_distinct(map: &Map<i64>, pos: Pose) -> i64 {
    let mut queue = VecDeque::new();
    let mut reachable = Vec::new();
    queue.push_back(pos);
    while let Some(current_pos) = queue.pop_front() {
        let Some(&current_height) = map.get(current_pos) else {
            continue;
        };

        current_pos
            .adjacent()
            .iter()
            .filter_map(|&adjacent_pos| map.get(adjacent_pos).map(|&height| (adjacent_pos, height)))
            .filter(|(_, height)| *height == current_height + 1)
            .for_each(|(adjacent_pos, height)| {
                if height == 9 {
                    reachable.push(adjacent_pos);
                } else {
                    queue.push_back(adjacent_pos);
                }
            });
    }
    reachable.len() as i64
}

pub fn part2(input: &str) -> Result<String, String> {
    let map = parse(input).map_err(|e| e.to_string())?.1;
    let mut scores = Vec::new();
    for y in 0..map.height() {
        for x in 0..map.width() {
            let pos = Pose::new_xy(x as i64, y as i64);
            if map.get(pos).is_some_and(|v| *v == 0) {
                scores.push(trailhead_score_distinct(&map, pos));
            }
        }
    }
    Ok(scores.iter().sum::<i64>().to_string())
}
