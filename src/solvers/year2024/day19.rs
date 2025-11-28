use nom::{
    IResult, Parser,
    bytes::complete::tag,
    character::complete::{line_ending, one_of},
    combinator::{all_consuming, map},
    multi::{many1, separated_list1},
};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
enum Color {
    White,
    Blue,
    Black,
    Red,
    Green,
}

type Pattern = Vec<Color>;

fn parse_color(input: &str) -> IResult<&str, Color> {
    map(one_of("wubrg"), |v| match v {
        'w' => Color::White,
        'u' => Color::Blue,
        'b' => Color::Black,
        'r' => Color::Red,
        'g' => Color::Green,
        _ => unreachable!("one_of only returns wubrg"),
    })
    .parse(input)
}

fn parse_pattern(input: &str) -> IResult<&str, Pattern> {
    many1(parse_color).parse(input.trim())
}

fn parse(input: &str) -> Result<(Vec<Pattern>, Vec<Pattern>), String> {
    let (input, patterns) = separated_list1(tag(","), parse_pattern)
        .parse(input.trim())
        .map_err(|e| e.to_string())?;

    let (_, designs) = all_consuming(separated_list1(line_ending, parse_pattern))
        .parse(input.trim())
        .map_err(|e| e.to_string())?;

    Ok((patterns, designs))
}

type PatternSlice<'a> = &'a [Color];

fn possible(rest_design: PatternSlice, patterns: &[Pattern]) -> bool {
    if rest_design.is_empty() {
        return true;
    }

    for pattern in patterns {
        if rest_design.starts_with(pattern) && possible(&rest_design[pattern.len()..], patterns) {
            return true;
        }
    }

    false
}

pub fn part1(input: &str) -> Result<String, String> {
    let (patterns, designs) = parse(input)?;
    let possible_designs = designs.iter().filter(|d| possible(d, &patterns)).count();
    Ok(possible_designs.to_string())
}

pub fn part2(input: &str) -> Result<String, String> {
    Ok(1.to_string())
}
