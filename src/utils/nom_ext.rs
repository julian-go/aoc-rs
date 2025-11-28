use nom::{IResult, character::complete::one_of};

pub fn map_size(input: &str) -> (usize, usize) {
    let input = input.trim();
    let height = input.lines().count();
    let width = input.lines().next().map_or(0, str::len);
    (width, height)
}

pub fn one_digit(input: &str) -> IResult<&str, i64> {
    let (input, digit) = one_of("0123456789")(input)?;
    Ok((input, i64::from(digit.to_digit(10).unwrap())))
}
