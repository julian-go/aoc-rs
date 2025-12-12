use std::collections::{HashMap, HashSet};

use nom::{
    IResult, Parser,
    character::complete::{char, i64 as nom_i64, line_ending},
    multi::separated_list1,
    sequence::separated_pair,
};

fn parse(input: &str) -> IResult<&str, Vec<(i64, i64)>> {
    separated_list1(line_ending, separated_pair(nom_i64, char(','), nom_i64)).parse(input)
}

fn rect_size(a: (i64, i64), b: (i64, i64)) -> i64 {
    let width = (a.0 - b.0).abs() + 1;
    let height = (a.1 - b.1).abs() + 1;

    width * height
}

pub fn part1(input: &str) -> Result<String, String> {
    let (_, tiles) = parse(input).map_err(|e| e.to_string())?;

    let mut max_rectangle: i64 = 0;
    for i in 0..tiles.len() {
        for j in i..tiles.len() {
            max_rectangle = max_rectangle.max(rect_size(tiles[i], tiles[j]));
        }
    }

    Ok(max_rectangle.to_string())
}

#[derive(Clone, Copy, Debug, Default)]
struct Line {
    pub from: (i64, i64),
    pub to: (i64, i64),
}

impl Line {
    pub fn new(from: (i64, i64), to: (i64, i64)) -> Self {
        use std::cmp::Ordering;
        match from.0.cmp(&to.0) {
            Ordering::Equal => {
                if from.1 > to.1 {
                    Line { from: to, to: from }
                } else {
                    Line { from, to }
                }
            }
            Ordering::Greater => Line { from: to, to: from },
            Ordering::Less => Line { from, to },
        }
    }

    pub fn horizontal(&self) -> bool {
        self.from.1 == self.to.1
    }

    pub fn vertical(&self) -> bool {
        !self.horizontal()
    }

    pub fn parallel(&self, other: &Line) -> bool {
        (self.horizontal() && other.horizontal()) || (self.vertical() && other.vertical())
    }

    pub fn intersecting(&self, other: &Line) -> bool {
        if self.parallel(other) {
            return false;
        }

        let (h_line, v_line) = if self.horizontal() {
            (self, other)
        } else {
            (other, self)
        };

        let x_in_range = h_line.from.0 < v_line.from.0 && v_line.from.0 < h_line.to.0;
        let y_in_range = v_line.from.1 < h_line.from.1 && h_line.from.1 < v_line.to.1;

        x_in_range && y_in_range
    }
}

#[derive(Clone, Copy, Debug, Default)]
struct Rectangle {
    from: (i64, i64),
    to: (i64, i64),
}

fn rectangle_valid(from: (i64, i64), to: (i64, i64), lines: &[Line]) -> bool {
    let rect_lines = [
        Line::new(from, (to.0, from.1)), // top
        Line::new((to.0, from.1), to),   // right
        Line::new(to, (from.0, to.1)),   // bottom
        Line::new((from.0, to.1), from), // left
    ];

    lines.iter().all(|line| {
        rect_lines
            .iter()
            .all(|rect_line| !line.intersecting(rect_line))
    })
}

pub fn part2(input: &str) -> Result<String, String> {
    let (_, tiles) = parse(input).map_err(|e| e.to_string())?;

    let mut lines = Vec::new();
    for i in 0..tiles.len() {
        lines.push(Line::new(tiles[i], tiles[(i + 1) % tiles.len()]));
    }

    let mut max_rectangle: i64 = 0;
    for i in 0..tiles.len() {
        for j in i..tiles.len() {
            let size = rect_size(tiles[i], tiles[j]);
            if size > max_rectangle && rectangle_valid(tiles[i], tiles[j], &lines) {
                max_rectangle = size;
            }
        }
    }

    Ok(max_rectangle.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_intersecting() {
        // Parallel lines (horizontal)
        assert!(!Line::new((0, 0), (10, 0)).intersecting(&Line::new((0, 0), (10, 0))));
        assert!(!Line::new((0, 0), (10, 0)).intersecting(&Line::new((0, 5), (10, 5))));

        // Parallel lines (vertical)
        assert!(!Line::new((0, 0), (0, 10)).intersecting(&Line::new((5, 0), (5, 10))));

        // Lines sharing endpoints (touching, not intersecting)
        assert!(!Line::new((0, 0), (10, 0)).intersecting(&Line::new((0, 0), (0, 10))));
        assert!(!Line::new((0, 0), (10, 0)).intersecting(&Line::new((10, 0), (10, 10))));

        // Vertical line on horizontal line endpoint (touching, not intersecting)
        assert!(!Line::new((0, 0), (10, 0)).intersecting(&Line::new((1, 0), (1, 10))));
        assert!(!Line::new((0, 0), (10, 0)).intersecting(&Line::new((0, 0), (0, 10))));
        assert!(!Line::new((0, 0), (10, 0)).intersecting(&Line::new((10, 0), (10, 10))));

        // Actual intersections (crosses in the middle)
        assert!(Line::new((0, 0), (10, 0)).intersecting(&Line::new((1, -1), (1, 10))));
        assert!(Line::new((0, 0), (10, 0)).intersecting(&Line::new((5, -5), (5, 5))));
        assert!(Line::new((0, 0), (0, 10)).intersecting(&Line::new((-5, 5), (5, 5))));

        // Lines that don't intersect (perpendicular but not crossing)
        assert!(!Line::new((0, 0), (10, 0)).intersecting(&Line::new((15, -5), (15, 5))));
        assert!(!Line::new((0, 0), (10, 0)).intersecting(&Line::new((5, 5), (5, 10))));
        assert!(!Line::new((0, 0), (0, 10)).intersecting(&Line::new((5, 15), (10, 15))));

        // Edge cases with negative coordinates
        assert!(Line::new((-5, 0), (5, 0)).intersecting(&Line::new((0, -5), (0, 5))));
        assert!(!Line::new((-5, 0), (5, 0)).intersecting(&Line::new((6, -5), (6, 5))));
    }

    #[test]
    fn test_line_orientation() {
        let h_line = Line::new((0, 0), (10, 0));
        let v_line = Line::new((0, 0), (0, 10));

        assert!(h_line.horizontal());
        assert!(!h_line.vertical());
        assert!(!v_line.horizontal());
        assert!(v_line.vertical());
    }

    #[test]
    fn test_line_parallel() {
        assert!(Line::new((0, 0), (10, 0)).parallel(&Line::new((0, 5), (10, 5))));
        assert!(Line::new((0, 0), (0, 10)).parallel(&Line::new((5, 0), (5, 10))));
        assert!(!Line::new((0, 0), (10, 0)).parallel(&Line::new((0, 0), (0, 10))));
    }

    #[test]
    fn test_rect_size() {
        assert_eq!(rect_size((0, 0), (9, 9)), 100);
        assert_eq!(rect_size((0, 0), (0, 0)), 1);
        assert_eq!(rect_size((5, 5), (10, 10)), 36);
        assert_eq!(rect_size((10, 10), (5, 5)), 36);
    }
}
