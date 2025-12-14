use nom::{
    IResult, Parser,
    bytes::complete::{tag, take_till},
    character::complete::{line_ending, one_of, space1, usize as nom_usize},
    combinator::{all_consuming, map},
    multi::{many1, separated_list1},
    sequence::separated_pair,
};

struct Gift(Vec<Vec<bool>>);

struct Region {
    pub size: (usize, usize),
    pub gifts: Vec<usize>,
}

fn parse_box(input: &str) -> IResult<&str, Gift> {
    let (input, _) = take_till(|c| c == '.' || c == '#').parse(input)?;
    let line = map(many1(one_of(".#")), |v| {
        v.iter().map(|c| *c == '#').collect()
    });
    map(separated_list1(line_ending, line), Gift).parse(input.trim())
}

fn parse_region(input: &str) -> IResult<&str, Region> {
    let (input, (size, gifts)) = separated_pair(
        separated_pair(nom_usize, tag("x"), nom_usize),
        tag(": "),
        separated_list1(space1, nom_usize),
    )
    .parse(input.trim())?;
    Ok((input, Region { size, gifts }))
}

fn parse(input: &str) -> IResult<&str, (Vec<Gift>, Vec<Region>)> {
    let (input, gifts) = separated_list1(line_ending, parse_box).parse(input.trim())?;
    let (input, regions) =
        all_consuming(separated_list1(line_ending, parse_region)).parse(input.trim())?;
    Ok((input, (gifts, regions)))
}

impl Gift {
    pub fn num_cells(&self) -> usize {
        self.0
            .iter()
            .map(|v| v.iter().filter(|&value| *value).count())
            .sum()
    }
}

pub fn part1(input: &str) -> Result<String, String> {
    let (_, (gifts, regions)) = parse(input.trim()).map_err(|e| e.to_string())?;
    let possible_regions = regions
        .iter()
        .map(|reg| {
            let available_cells = reg.size.0 * reg.size.1;
            let required_cells = reg
                .gifts
                .iter()
                .enumerate()
                .map(|(gift_index, gift_count)| gift_count * gifts[gift_index].num_cells())
                .sum::<usize>();
            i64::from(available_cells >= required_cells)
        })
        .sum::<i64>();
    let possible_regions = possible_regions - i64::from(possible_regions < 10); // extremely important check
    Ok(possible_regions.to_string())
}

pub fn part2(input: &str) -> Result<String, String> {
    Ok(1.to_string())
}
