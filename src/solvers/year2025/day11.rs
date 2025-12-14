use std::collections::HashMap;

use nom::{
    IResult, Parser,
    bytes::complete::tag,
    character::complete::{alpha1, line_ending, space1},
    combinator::{all_consuming, map},
    multi::separated_list1,
    sequence::separated_pair,
};

use nalgebra::DMatrix;

fn parse_device(input: &str) -> IResult<&str, String> {
    map(alpha1, String::from).parse(input)
}

fn parse_line(input: &str) -> IResult<&str, (String, Vec<String>)> {
    separated_pair(
        parse_device,
        tag(": "),
        separated_list1(space1, parse_device),
    )
    .parse(input)
}

fn parse(input: &str) -> IResult<&str, Vec<(String, Vec<String>)>> {
    all_consuming(separated_list1(line_ending, parse_line)).parse(input.trim())
}

type DeviceTree = Vec<Vec<usize>>;

fn to_device_tree(devices: Vec<(String, Vec<String>)>) -> (usize, usize, DeviceTree) {
    let mut device_map = HashMap::new();
    for (i, (device, _)) in devices.iter().enumerate() {
        device_map.insert(device.clone(), i);
    }
    device_map.insert("out".into(), device_map.len());

    let mut tree: DeviceTree = vec![vec![]; devices.len()];
    for (i, (_, outputs)) in devices.into_iter().enumerate() {
        tree[i] = outputs.into_iter().map(|dev| device_map[&dev]).collect();
    }
    tree.push(vec![]);

    (
        *device_map.get("you").unwrap(),
        *device_map.get("out").unwrap(),
        tree,
    )
}

pub fn part1(input: &str) -> Result<String, String> {
    let (_, devices) = parse(input).map_err(|e| e.to_string())?;
    let (start_idx, goal_idx, tree) = to_device_tree(devices);
    let mut adjacency = DMatrix::<usize>::zeros(tree.len(), tree.len());
    for (i, row) in tree.iter().enumerate() {
        for j in row {
            adjacency[(i, *j)] = 1;
        }
    }

    let original = adjacency.clone();
    let mut total_paths = DMatrix::<usize>::zeros(tree.len(), tree.len());
    let mut current_power = adjacency.clone();

    // Sum A + A² + A³ + ... until A^k becomes zero matrix
    loop {
        total_paths += current_power.clone();
        let next_power = current_power * &original;

        // Check if we've reached the zero matrix (no more paths)
        if next_power.iter().all(|&x| x == 0) {
            break;
        }

        current_power = next_power;
    }

    let num_paths = total_paths[(start_idx, goal_idx)];
    Ok(num_paths.to_string())
}

pub fn part2(input: &str) -> Result<String, String> {
    Ok(1.to_string())
}
