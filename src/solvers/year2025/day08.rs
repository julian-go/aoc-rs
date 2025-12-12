use std::collections::HashSet;

use nom::{
    IResult, Parser,
    character::complete::{char, i64 as nom_i64, line_ending},
    combinator::{all_consuming, map},
    multi::separated_list1,
};

#[derive(Debug, Default, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct Junction {
    pub x: i64,
    pub y: i64,
    pub z: i64,
}

impl Junction {
    fn square_euclidean(&self, other: &Junction) -> i64 {
        (self.x - other.x).pow(2) + (self.y - other.y).pow(2) + (self.z - other.z).pow(2)
    }
}

fn parse(input: &str) -> IResult<&str, Vec<Junction>> {
    all_consuming(separated_list1(
        line_ending,
        map(
            (nom_i64, char(','), nom_i64, char(','), nom_i64),
            |(x, _, y, _, z)| Junction { x, y, z },
        ),
    ))
    .parse(input.trim())
}

fn sorted_distances(junctions: &[Junction]) -> Vec<(usize, usize, i64)> {
    let mut networks = Vec::with_capacity(junctions.len() * junctions.len());
    for (i, junction) in junctions.iter().enumerate() {
        for (j, other_junction) in junctions.iter().enumerate().skip(i + 1) {
            networks.push((i, j, junction.square_euclidean(other_junction)));
        }
    }
    networks.sort_by(|a, b| a.2.cmp(&b.2));
    networks
}

type Network = HashSet<usize>;

/// This function returns true if the nodes connected something to the large network, so either
/// - Two networks were merged
/// - A new node was added to the large network
///
/// It's only used in part 2
fn connect_closest(j1: usize, j2: usize, networks: &mut Vec<Network>) -> bool {
    let i1 = networks.iter().position(|c| c.contains(&j1));
    let i2 = networks.iter().position(|c| c.contains(&j2));

    if let (Some(i1), Some(i2)) = (i1, i2) {
        if i1 == i2 {
            false
        } else {
            // There are two seperate networks, that need to be connected
            let n2 = networks[i2].clone();
            networks[i1].extend(n2);
            networks.remove(i2);
            true
        }
    } else if let Some(i1) = i1 {
        networks[i1].insert(j2);
        true
    } else if let Some(i2) = i2 {
        networks[i2].insert(j1);
        true
    } else {
        networks.push(HashSet::from([j1, j2]));
        false
    }
}

pub fn part1(input: &str) -> Result<String, String> {
    let (_, junctions) = parse(input).map_err(|e| e.to_string())?;
    let num_connections = if junctions.len() < 30 { 10 } else { 1000 };
    let distances = sorted_distances(&junctions);

    let mut networks: Vec<Network> = Vec::new();
    for (i, j, _) in distances.into_iter().take(num_connections) {
        connect_closest(i, j, &mut networks);
    }

    let mut total = 1;
    for _ in 0..3 {
        let (max_idx, _) = networks
            .iter()
            .map(Network::len)
            .enumerate()
            .max_by_key(|(_, v)| *v)
            .unwrap();
        total *= networks.remove(max_idx).len();
    }

    Ok(total.to_string())
}

pub fn part2(input: &str) -> Result<String, String> {
    let (_, junctions) = parse(input).map_err(|e| e.to_string())?;
    let distances = sorted_distances(&junctions);

    let mut networks: Vec<Network> = Vec::new();
    let mut last_connected = (0, 0);
    for (i, j, _) in distances {
        if connect_closest(i, j, &mut networks) {
            last_connected = (i, j);
        }
    }

    let result = junctions[last_connected.0].x * junctions[last_connected.1].x;
    Ok(result.to_string())
}
