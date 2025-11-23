use nom::{
    IResult,
    character::complete::{i64 as nom_i64, one_of},
};

pub fn map_size(input: &str) -> (usize, usize) {
    let input = input.trim();
    let height = input.lines().count();
    let width = input.lines().next().map(|line| line.len()).unwrap_or(0);
    (width, height)
}

pub fn one_digit(input: &str) -> IResult<&str, i64> {
    let (input, digit) = one_of("0123456789")(input)?;
    Ok((input, digit.to_digit(10).unwrap() as i64))
}
