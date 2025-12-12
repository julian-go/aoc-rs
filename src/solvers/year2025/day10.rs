use std::collections::HashMap;

use nom::{
    IResult, Parser,
    character::complete::{char, line_ending, one_of, space1, usize as nom_usize},
    combinator::{all_consuming, map},
    multi::{many1, separated_list1},
    sequence::delimited,
};

type Button = Vec<usize>;
type Joltage = Vec<usize>;

#[derive(Debug, Clone, Default)]
struct Machine {
    lights: Vec<bool>,
    buttons: Vec<Button>,
    joltage: Joltage,
}

fn parse_lights(input: &str) -> IResult<&str, Vec<bool>> {
    let light = map(one_of(".#"), |c| c == '#');
    delimited(char('['), many1(light), char(']')).parse(input)
}

fn parse_button(input: &str) -> IResult<&str, Button> {
    delimited(char('('), separated_list1(char(','), nom_usize), char(')')).parse(input)
}

fn parse_joltage(input: &str) -> IResult<&str, Button> {
    delimited(char('{'), separated_list1(char(','), nom_usize), char('}')).parse(input)
}

fn parse_machine(input: &str) -> IResult<&str, Machine> {
    let (input, lights) = parse_lights(input.trim())?;
    let (input, buttons) = separated_list1(space1, parse_button).parse(input.trim())?;
    let (input, joltage) = parse_joltage(input.trim())?;

    Ok((
        input,
        Machine {
            lights,
            buttons,
            joltage,
        },
    ))
}

fn parse(input: &str) -> IResult<&str, Vec<Machine>> {
    all_consuming(separated_list1(line_ending, parse_machine)).parse(input.trim())
}

/// (State of the lights, Has button been pressed)
type DpState = (Vec<bool>, Vec<bool>);
type DpCost = i64;

/// We pretend we are in the target state and want to turn all the light off,
/// the button sequence is the same
/// It doesnt make sense to push any button twice
fn dp(memo: &mut HashMap<DpState, DpCost>, state: &DpState, buttons: &[Button]) -> i64 {
    if memo.contains_key(state) {
        return memo[state];
    }

    let value = if state.0.iter().all(|v| !*v) {
        // if all lights are off we have arrived at the solution
        0
    } else if state.1.iter().all(|v| *v) {
        // all buttons have been pressed
        i64::MAX / 2
    } else {
        // Generate the set of all states we can reach from here
        let mut possible_states: Vec<DpState> = buttons
            .iter()
            .enumerate()
            .filter_map(|(i, b)| {
                if state.1[i] {
                    None
                } else {
                    let mut new_state = state.clone();
                    new_state.1[i] = true; // set button to pressed
                    for light_index in b {
                        new_state.0[*light_index] = !new_state.0[*light_index];
                    }
                    Some(new_state)
                }
            })
            .collect();

        // Sort, such that we greedily turn on lights
        possible_states.sort_unstable_by(|a, b| {
            let active_lights_a = a.0.iter().filter(|&av| *av).count();
            let active_lights_b = b.0.iter().filter(|&bv| *bv).count();
            active_lights_a.cmp(&active_lights_b)
        });

        possible_states
            .into_iter()
            .map(|state| dp(memo, &state, buttons) + 1)
            .min()
            .unwrap()
    };
    memo.insert(state.clone(), value);
    value
}

fn solve(machine: &Machine) -> i64 {
    let mut memo = HashMap::new();
    let state = (machine.lights.clone(), vec![false; machine.buttons.len()]);
    let value = dp(&mut memo, &state, &machine.buttons);
    value
}

pub fn part1(input: &str) -> Result<String, String> {
    let (_, machines) = parse(input).map_err(|e| e.to_string())?;
    let total: i64 = machines.iter().map(solve).sum();
    Ok(total.to_string())
}

pub fn part2(input: &str) -> Result<String, String> {
    Ok(1.to_string())
}
