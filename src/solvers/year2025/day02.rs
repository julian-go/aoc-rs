use nom::{
    IResult, Parser, bytes::complete::tag, character::complete::i64 as nom_i64,
    combinator::all_consuming, multi::separated_list1, sequence::separated_pair,
};

fn parse_interval(input: &str) -> IResult<&str, (i64, i64)> {
    separated_pair(nom_i64, tag("-"), nom_i64).parse(input)
}

fn parse(input: &str) -> Result<Vec<(i64, i64)>, String> {
    let (_, intervals) = all_consuming(separated_list1(tag(","), parse_interval))
        .parse(input.trim())
        .map_err(|e| e.to_string())?;
    Ok(intervals)
}

fn count_digits(i: i64) -> usize {
    let mut i = i / 10;
    let mut digits = 1;
    while i > 0 {
        digits += 1;
        i /= 10;
    }
    digits
}

fn at(i: i64, idx: usize) -> Option<i64> {
    let idx = idx as i64;
    let d = count_digits(i) as i64;
    if idx >= d {
        None
    } else {
        let div = (d - 1 - idx) as u32;
        let doi = (i / 10_i64.pow(div)) % 10;
        Some(doi)
    }
}

fn invalid(i: i64) -> bool {
    let d = count_digits(i);
    if d % 2 == 0 {
        let mut valid = false;
        let mid = d / 2;
        for idx in 0..mid {
            if at(i, idx) != at(i, mid + idx) {
                valid = true;
                break;
            }
        }
        !valid
    } else {
        false
    }
}

pub fn part1(input: &str) -> Result<String, String> {
    let intervals = parse(input)?;
    let mut sum = 0;
    for interval in &intervals {
        for i in interval.0..=interval.1 {
            if invalid(i) {
                sum += i;
            }
        }
    }
    Ok(sum.to_string())
}

fn is_prime(candidate: usize) -> bool {
    (2..candidate).all(|i| candidate % i != 0)
}

fn grouped_invalid(num: i64, groupings: &[usize]) -> bool {
    let num_digits = count_digits(num);
    for &num_groups in groupings {
        if num_groups > num_digits {
            break;
        }

        if num_digits % num_groups != 0 {
            continue;
        }

        let stride = num_digits / num_groups;
        let identical = (1..num_groups).all(|group| {
            let group_start = group * stride;
            (0..stride).all(|i| at(num, i) == at(num, group_start + i))
        });

        if identical {
            return true;
        }
    }
    false
}

pub fn part2(input: &str) -> Result<String, String> {
    let intervals = parse(input)?;

    // We only need to check prime groupings up to the number of digits in an i64
    let groupings: Vec<usize> = (2..=count_digits(i64::MAX))
        .filter(|&n| is_prime(n))
        .collect();

    let sum: i64 = intervals
        .iter()
        .map(|interval| {
            (interval.0..=interval.1)
                .map(|i| if grouped_invalid(i, &groupings) { i } else { 0 })
                .sum::<i64>()
        })
        .sum();

    Ok(sum.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_digits() {
        assert_eq!(count_digits(0), 1);
        assert_eq!(count_digits(1), 1);
        assert_eq!(count_digits(5), 1);
        assert_eq!(count_digits(10), 2);
        assert_eq!(count_digits(123), 3);
        assert_eq!(count_digits(112_312_323), 9);
    }

    #[test]
    fn test_at() {
        assert_eq!(at(123_456, 0), Some(1));
        assert_eq!(at(123_456, 1), Some(2));
        assert_eq!(at(123_456, 2), Some(3));
        assert_eq!(at(123_456, 3), Some(4));
        assert_eq!(at(123_456, 4), Some(5));
        assert_eq!(at(123_456, 5), Some(6));
    }

    #[test]
    fn test_invalid() {
        assert!(!invalid(123_456));
        assert!(!invalid(123_451_236));
        assert!(!invalid(1_234_256));
        assert!(!invalid(12_344_356));
        assert!(!invalid(1));
        assert!(invalid(11));
        assert!(invalid(1_111));
        assert!(invalid(110_110));
        assert!(invalid(11_041_104));
    }

    #[test]
    fn test_invalid_2() {
        let groupings = vec![2, 3, 5, 7, 11, 13, 17, 19];
        assert!(grouped_invalid(1_188_511_885, &groupings));
        assert!(grouped_invalid(565_656, &groupings));
    }

    #[test]
    fn test_is_prime() {
        assert!(is_prime(1));
        assert!(is_prime(2));
        assert!(is_prime(3));
        assert!(!is_prime(4));
    }
}
