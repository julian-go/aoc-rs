use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

/// A vector is a 2D vector in space with an x and a y component.
#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
pub struct Vector {
    pub x: i64,
    pub y: i64,
}

impl Vector {
    pub fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }

    pub fn manhattan(&self) -> i64 {
        self.x + self.y
    }
}

impl From<Vector> for (usize, usize) {
    fn from(v: Vector) -> (usize, usize) {
        (v.x as usize, v.y as usize)
    }
}

impl Add for Vector {
    type Output = Vector;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl AddAssign for Vector {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Sub for Vector {
    type Output = Vector;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl SubAssign for Vector {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl<T> Add<T> for Vector
where
    T: Copy,
    i64: Add<T, Output = i64>,
{
    type Output = Vector;

    fn add(self, rhs: T) -> Self::Output {
        Self {
            x: self.x + rhs,
            y: self.y + rhs,
        }
    }
}

impl<T> Sub<T> for Vector
where
    T: Copy,
    i64: Sub<T, Output = i64>,
{
    type Output = Vector;

    fn sub(self, rhs: T) -> Self::Output {
        Self {
            x: self.x - rhs,
            y: self.y - rhs,
        }
    }
}

impl<T> Mul<T> for Vector
where
    T: Copy,
    i64: Mul<T, Output = i64>,
{
    type Output = Vector;

    fn mul(self, rhs: T) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl<T> MulAssign<T> for Vector
where
    T: Copy,
    i64: MulAssign<T>,
{
    fn mul_assign(&mut self, rhs: T) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

impl<T> Div<T> for Vector
where
    T: Copy,
    i64: Div<T, Output = i64>,
{
    type Output = Vector;

    fn div(self, rhs: T) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

impl<T> DivAssign<T> for Vector
where
    T: Copy,
    i64: DivAssign<T>,
{
    fn div_assign(&mut self, rhs: T) {
        self.x /= rhs;
        self.y /= rhs;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_vectors() {
        assert_eq!(
            Vector { x: 1, y: 5 } + Vector { x: -3, y: 4 },
            Vector { x: -2, y: 9 }
        )
    }

    #[test]
    fn subtract_vectors() {
        assert_eq!(
            Vector { x: 1, y: 5 } - Vector { x: -3, y: 4 },
            Vector { x: 4, y: 1 }
        )
    }

    #[test]
    fn add_integer() {
        assert_eq!(Vector { x: 1, y: 5 } + -1, Vector { x: 0, y: 4 })
    }

    #[test]
    fn sub_integer() {
        assert_eq!(Vector { x: 1, y: 5 } - -4, Vector { x: 5, y: 9 })
    }

    #[test]
    fn mul_integer() {
        assert_eq!(Vector { x: 1, y: 5 } * -4, Vector { x: -4, y: -20 })
    }

    #[test]
    fn div_integer() {
        assert_eq!(Vector { x: 1, y: 5 } / -4, Vector { x: 0, y: -1 })
    }

    #[test]
    fn assignment_ops() {
        let mut v = Vector { x: 2, y: 8 };
        v += Vector { x: 1, y: -3 };
        assert_eq!(v, Vector { x: 3, y: 5 });
        v -= Vector { x: -3, y: 2 };
        assert_eq!(v, Vector { x: 6, y: 3 });
        v *= 2;
        assert_eq!(v, Vector { x: 12, y: 6 });
        v /= 3;
        assert_eq!(v, Vector { x: 4, y: 2 });
    }
}
