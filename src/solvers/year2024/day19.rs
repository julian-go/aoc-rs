use nom::{
    IResult, Parser,
    bytes::complete::tag,
    character::complete::{alpha1, line_ending, one_of},
    combinator::{all_consuming, map},
    multi::{many1, separated_list1},
};

use std::collections::HashMap;

type Pattern<'a> = &'a str;

fn parse_color(input: &str) -> IResult<&str, char> {
    one_of("wubrg").parse(input)
}

fn parse_pattern(input: &str) -> IResult<&str, &str> {
    alpha1(input.trim())
}

fn parse(input: &str) -> Result<(Vec<Pattern>, Vec<Pattern>), String> {
    let (input, patterns) = separated_list1(tag(","), parse_pattern)
        .parse(input.trim())
        .map_err(|e| e.to_string())?
        .clone();

    let (_, designs) = all_consuming(separated_list1(line_ending, parse_pattern))
        .parse(input.trim())
        .map_err(|e| e.to_string())?;

    Ok((patterns, designs))
}

type PatternSlice<'a> = &'a str;

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

fn evaluate_design(memo: &mut HashMap<String, i32>, design: Pattern) -> i32 {
    if design.is_empty() {
        return 1;
    }

    if let Some(value) = memo.get(design) {
        return *value;
    }

    let mut num_variants = 0;
    let keys: Vec<String> = memo.keys().cloned().collect();
    for k in keys {
        if design.starts_with(&k) {
            let num = evaluate_design(memo, &design[k.len()..]);
            if num > 0 {
                println!("{design} can be made in {num}!");
            }
            num_variants += num;
        } else {
            // println!("{design} -> {k}");
        }
    }
    // println!("{design} -> {num_variants}");
    memo.insert(design.to_string(), num_variants);

    num_variants

    // let subdesign = &design[index..];
    // if memo.contains(subdesign)
    // for pattern in patterns {
    //     if subdesign.starts_with(pattern) {
    //         possible_count(count, memo, index + pattern.len(), design, patterns);
    //     }
    // }
}

pub fn part2(input: &str) -> Result<String, String> {
    let (patterns, designs) = parse(input)?;
    let mut memo = HashMap::new();
    for pattern in &patterns {
        memo.insert((*pattern).to_string(), 1);
    }
    println!("{memo:#?}");
    let mut num_designs = 0;
    for design in &designs {
        if *design == "gbbr" {
            let num = evaluate_design(&mut memo, &design);
            println!("{design} -> {num}");
        }
    }

    // let total_count: i64 = designs
    //     .iter()
    //     .map(|d| {
    //         let mut count = 0;
    //         possible_count(&mut count, d, &patterns);
    //         count
    //     })
    //     .sum();
    // Ok(total_count.to_string())
    Ok(num_designs.to_string())
}
