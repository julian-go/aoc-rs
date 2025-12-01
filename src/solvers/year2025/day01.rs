use nom::{
    IResult, Parser,
    character::complete::{i32 as nom_i32, line_ending, one_of},
    combinator::all_consuming,
    multi::separated_list1,
};

fn parse_rotation(input: &str) -> IResult<&str, i32> {
    let (input, (dir, count)) = (one_of("LR"), nom_i32).parse(input)?;
    let sign = if dir == 'L' { -1 } else { 1 };
    Ok((input, sign * count))
}

fn parse(input: &str) -> Result<Vec<i32>, String> {
    let (_, rotations) = all_consuming(separated_list1(line_ending, parse_rotation))
        .parse(input.trim())
        .map_err(|e| e.to_string())?;
    Ok(rotations)
}

pub fn part1(input: &str) -> Result<String, String> {
    let rotations = parse(input)?;
    let mut zero_count = 0;
    let mut current = 50;
    for increment in rotations {
        current += increment;
        if current % 100 == 0 {
            zero_count += 1;
        }
    }
    Ok(zero_count.to_string())
}

fn div_floor(a: i32, b: i32) -> i32 {
    (a as f32 / b as f32).floor() as i32
}

fn to_turn(value: i32) -> i32 {
    div_floor(value, 100)
}

pub fn part2(input: &str) -> Result<String, String> {
    let rotations = parse(input)?;
    let mut zero_count = 0;
    let mut current: i32 = 50;
    for increment in rotations {
        let previous = current;
        current += increment;
        zero_count += (to_turn(current) - to_turn(previous)).abs();

        // only count decrementing to zero, not from it
        if increment < 0 {
            if previous.rem_euclid(100) == 0 {
                zero_count -= 1;
            } else if current.rem_euclid(100) == 0 {
                zero_count += 1;
            }
        }
    }
    Ok(zero_count.to_string())
}
