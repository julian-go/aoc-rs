use std::fmt;

use crate::utils::Vector;

pub struct Matrix<T> {
    width: usize,
    height: usize,
    data: Vec<T>,
}

impl<T> Matrix<T> {
    fn xy_to_index(&self, x: usize, y: usize) -> usize {
        // this is a row major implementation
        x + self.width * y
    }

    fn index_to_xy(&self, index: usize) -> (usize, usize) {
        (index % self.width, index / self.width)
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn get<P>(&self, pos: P) -> Option<&T>
    where
        P: Into<(usize, usize)> + Copy,
    {
        let (x, y) = pos.into();

        if x < self.width && y < self.height {
            let index = self.xy_to_index(x, y);
            Some(&self.data[index])
        } else {
            None
        }
    }

    pub fn get_mut<P>(&mut self, pos: P) -> Option<&mut T>
    where
        P: Into<(usize, usize)> + Copy,
    {
        let (x, y) = pos.into();

        if x < self.width && y < self.height {
            let index = self.xy_to_index(x, y);
            Some(&mut self.data[index])
        } else {
            None
        }
    }
}

impl<T> Matrix<T>
where
    T: Default + Clone + Copy,
{
    pub fn new(width: usize, height: usize) -> Self {
        Matrix {
            width,
            height,
            data: vec![T::default(); height * width],
        }
    }

    pub fn from_vec(width: usize, data: Vec<T>) -> Result<Self, String> {
        if data.len() % width != 0 {
            return Err("Data length is not a multiple of width".to_string());
        }
        let height = data.len() / width;
        Ok(Matrix {
            width,
            height,
            data,
        })
    }

    pub fn map<U, F>(self, f: F) -> Matrix<U>
    where
        F: FnMut(T) -> U,
    {
        Matrix {
            width: self.width,
            height: self.height,
            data: self.data.into_iter().map(f).collect(),
        }
    }

    pub fn iter_positions(&self) -> impl Iterator<Item = (Vector, T)> {
        self.data.iter().enumerate().map(|(i, val)| {
            let (x, y) = (i % self.width, i / self.width);
            (Vector::new(x as i64, y as i64), *val)
        })
    }

    pub fn iter_positions_mut(&mut self) -> impl Iterator<Item = (Vector, T)> {
        self.data.iter_mut().enumerate().map(|(i, val)| {
            let (x, y) = (i % self.width, i / self.width);
            (Vector::new(x as i64, y as i64), *val)
        })
    }
}

impl<T> fmt::Debug for Matrix<T>
where
    T: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Map {}x{} [", self.width, self.height)?;
        for y in 0..self.height {
            write!(f, "  ")?;
            for x in 0..self.width {
                let index = self.xy_to_index(x, y);
                if x > 0 {
                    write!(f, " ")?;
                }
                write!(f, "{:?}", self.data[index])?;
            }
            writeln!(f)?;
        }
        write!(f, "]")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_map() {
        let mut map = Matrix::<i64>::new(4, 4);
        let value = map.get_mut((1, 1));
        assert!(value.is_some());
        let value = value.unwrap();
        *value += 10;
        println!("{map:?}");
    }
}
