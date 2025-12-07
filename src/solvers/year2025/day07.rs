use std::collections::{HashMap, HashSet};

pub fn parse(input: &str) -> (Vec<HashSet<i64>>, i64) {
    let mut rows = Vec::new();
    let mut start = 0;
    for (i_line, line) in input.trim().lines().enumerate() {
        if i_line % 2 == 1 {
            continue;
        }
        let mut splitters = HashSet::new();
        for (i_c, c) in line.chars().enumerate() {
            match c {
                'S' => start = i_c.try_into().unwrap(),
                '^' => {
                    splitters.insert(i_c.try_into().unwrap());
                }
                _ => (),
            }
        }
        if i_line > 0 {
            rows.push(splitters);
        }
    }
    (rows, start)
}

#[allow(clippy::unnecessary_wraps)]
pub fn part1(input: &str) -> Result<String, String> {
    let (rows, start) = parse(input);
    let mut tachs = HashSet::from([start]);
    let mut splits = 0;

    for splitters in rows {
        let mut new_tachs = HashSet::new();
        for tach in tachs {
            if splitters.contains(&tach) {
                new_tachs.insert(tach - 1);
                new_tachs.insert(tach + 1);
                splits += 1;
            } else {
                new_tachs.insert(tach);
            }
        }
        tachs = new_tachs;
    }
    Ok(splits.to_string())
}

#[allow(clippy::unnecessary_wraps)]
pub fn part2(input: &str) -> Result<String, String> {
    let (rows, start) = parse(input);
    let mut tachs = HashMap::from([(start, 1_i64)]);
    for splitters in rows {
        let mut new_tachs = HashMap::new();
        for (pos, num) in tachs {
            if splitters.contains(&pos) {
                *new_tachs.entry(pos + 1).or_insert(0) += num;
                *new_tachs.entry(pos - 1).or_insert(0) += num;
            } else {
                *new_tachs.entry(pos).or_insert(0) += num;
            }
        }
        tachs = new_tachs;
    }
    let total: i64 = tachs.values().sum();
    Ok(total.to_string())
}
