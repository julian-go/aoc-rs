use std::cmp;

/// An interval is exlusive [a, b)
#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
pub struct Interval {
    a: i64,
    b: i64,
}

impl Interval {
    pub fn new(from: i64, to: i64) -> Self {
        Interval { a: from, b: to }
    }

    pub fn includes(&self, value: i64) -> bool {
        value >= self.a && value < self.b
    }

    pub fn len(&self) -> usize {
        (self.b - self.a).max(0) as usize
    }

    pub fn empty(&self) -> bool {
        self.a == self.b
    }

    fn overlaps_positive(self, other: Interval) -> bool {
        self.a <= other.a && other.a < self.b && self.b <= other.b
    }

    fn overlaps_negative(self, other: Interval) -> bool {
        other.overlaps_positive(self)
    }

    pub fn overlaps(self, other: Interval) -> bool {
        self.overlaps_positive(other)
            || self.overlaps_negative(other)
            || self.contains(other)
            || other.contains(self)
    }

    pub fn union(self, other: Interval) -> Self {
        Interval::new(cmp::min(self.a, other.a), cmp::max(self.b, other.b))
    }

    pub fn contains(self, other: Interval) -> bool {
        self.a <= other.a && self.b >= other.b
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_overlap() {
        assert!(Interval::new(0, 10).overlaps(Interval::new(0, 10)));
        assert!(Interval::new(0, 10).overlaps(Interval::new(1, 10)));
        assert!(Interval::new(0, 10).overlaps(Interval::new(1, 11)));
        assert!(Interval::new(0, 10).overlaps(Interval::new(9, 11)));
        assert!(!Interval::new(0, 10).overlaps(Interval::new(10, 11)));
        assert!(Interval::new(0, 10).overlaps(Interval::new(4, 6)));
        assert!(Interval::new(0, 10).overlaps(Interval::new(-5, 1)));
        assert!(!Interval::new(0, 10).overlaps(Interval::new(-5, 0)));
    }
}
