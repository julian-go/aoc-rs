use std::fmt;

pub struct Map<T> {
    width: usize,
    height: usize,
    data: Vec<T>,
}

impl<T> Map<T> {
    fn xy_to_index(&self, x: usize, y: usize) -> usize {
        // this is a row major implementation
        x + self.width * y
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

impl<T> Map<T>
where
    T: Default + Clone,
{
    pub fn new(width: usize, height: usize) -> Self {
        Map {
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
        Ok(Map {
            width,
            height,
            data,
        })
    }
}

impl<T> fmt::Debug for Map<T>
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
        let mut map = Map::<i64>::new(4, 4);
        let value = map.get_mut((1, 1));
        assert!(value.is_some());
        let value = value.unwrap();
        *value += 10;
        println!("{map:?}");
    }
}
