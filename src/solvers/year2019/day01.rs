use nom::{
    IResult, Parser,
    character::complete::{i32 as nom_i32, line_ending},
    combinator::all_consuming,
    multi::separated_list1,
};

fn parse(input: &str) -> IResult<&str, Vec<i32>> {
    // remove trailing whitespace
    let input = input.trim();
    all_consuming(separated_list1(line_ending, nom_i32)).parse(input)
}

pub fn part1(input: &str) -> Result<String, String> {
    let (_, masses) = parse(input).map_err(|e| e.to_string())?;

    let total_fuel: i32 = masses.iter().map(|v| v / 3 - 2).sum();

    Ok(total_fuel.to_string())
}

pub fn part2(input: &str) -> Result<String, String> {
    let (_, masses) = parse(input).map_err(|e| e.to_string())?;

    let total_fuel: i32 = masses
        .iter()
        .map(|&mass| {
            std::iter::successors(Some(mass), |&m| {
                let fuel = m / 3 - 2;
                (fuel > 0).then_some(fuel)
            })
            .skip(1)
            .sum::<i32>()
        })
        .sum();

    Ok(total_fuel.to_string())
}
