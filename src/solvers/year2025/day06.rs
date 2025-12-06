use nom::{
    IResult, Parser,
    character::complete::{anychar, i64 as nom_i64, line_ending, multispace1, space0, space1},
    combinator::all_consuming,
    multi::separated_list1,
    sequence::separated_pair,
};

fn do_math(op: char, values: Vec<i64>) -> i64 {
    match op {
        '*' => values.into_iter().product::<i64>(),
        _ => values.into_iter().sum::<i64>(),
    }
}

mod part1 {
    use super::*;

    fn parse_operand_row(input: &str) -> IResult<&str, Vec<i64>> {
        separated_list1(space1, nom_i64).parse(input.trim())
    }

    fn parse_operator_row(input: &str) -> IResult<&str, Vec<char>> {
        separated_list1(space1, anychar).parse(input.trim())
    }

    pub fn parse(input: &str) -> Result<(Vec<Vec<i64>>, Vec<char>), String> {
        let (input, operands) = separated_list1(multispace1, parse_operand_row)
            .parse(input)
            .map_err(|e| e.to_string())?;
        let (_, operators) = all_consuming(parse_operator_row)
            .parse(input.trim())
            .map_err(|e| e.to_string())?;

        Ok((operands, operators))
    }
}

pub fn part1(input: &str) -> Result<String, String> {
    let (operands, operators) = part1::parse(input)?;

    let total: i64 = operators
        .into_iter()
        .enumerate()
        .map(|(i, operator)| {
            let values = operands.iter().map(|operand| operand[i]).collect();
            do_math(operator, values)
        })
        .sum();

    Ok(total.to_string())
}

pub fn transpose_input(input: &str) -> Result<String, String> {
    let n = input.lines().next().map_or(0, str::len);
    for line in input.lines().skip(1) {
        if line.len() != n {
            return Err(format!(
                "input does not have consistent length, expected {n} found {}",
                line.len()
            )
            .to_string());
        }
    }

    let lines: Vec<&str> = input.lines().collect();
    let mut transformed_input = String::new();
    for i in 0..n {
        for line in &lines {
            transformed_input.push(line.chars().nth(i).unwrap());
        }
        transformed_input.push('\n');
    }
    Ok(transformed_input)
}

mod part2 {
    use super::*;

    type Group = (Vec<i64>, char);

    fn parse_first_line(input: &str) -> IResult<&str, (i64, char)> {
        separated_pair(nom_i64, space0, anychar).parse(input)
    }

    fn parse_group(input: &str) -> IResult<&str, Group> {
        let (input, (first_value, op)) = parse_first_line(input.trim())?;
        let (input, mut values) =
            separated_list1((space0, line_ending, space0), nom_i64).parse(input.trim())?;
        values.insert(0, first_value);
        Ok((input, (values, op)))
    }

    pub fn parse(input: &str) -> Result<Vec<Group>, String> {
        let (_, groups) = all_consuming(separated_list1(multispace1, parse_group))
            .parse(input.trim())
            .map_err(|e| e.to_string())?;
        Ok(groups)
    }
}

pub fn part2(input: &str) -> Result<String, String> {
    let input = transpose_input(input)?;
    let groups = part2::parse(&input)?;
    let total: i64 = groups
        .into_iter()
        .map(|(values, op)| do_math(op, values))
        .sum();
    Ok(total.to_string())
}
