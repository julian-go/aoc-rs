use super::vector::Vector;

pub enum Orientation {
    North,
    East,
    South,
    West,
}

impl Orientation {
    pub fn to_vector(&self) -> Vector {
        match self {
            Orientation::North => Vector::new(0, -1),
            Orientation::East => Vector::new(1, 0),
            Orientation::South => Vector::new(0, 1),
            Orientation::West => Vector::new(-1, 0),
        }
    }

    pub fn turn_right(&mut self) {
        *self = match self {
            Orientation::North => Orientation::East,
            Orientation::East => Orientation::South,
            Orientation::South => Orientation::West,
            Orientation::West => Orientation::North,
        }
    }

    pub fn turn_left(&mut self) {
        *self = match self {
            Orientation::North => Orientation::West,
            Orientation::West => Orientation::South,
            Orientation::South => Orientation::East,
            Orientation::East => Orientation::North,
        }
    }

    pub fn turn_180(&mut self) {
        *self = match self {
            Orientation::North => Orientation::South,
            Orientation::East => Orientation::West,
            Orientation::South => Orientation::North,
            Orientation::West => Orientation::East,
        }
    }

    pub fn right(&self) -> Orientation {
        match self {
            Orientation::North => Orientation::East,
            Orientation::East => Orientation::South,
            Orientation::South => Orientation::West,
            Orientation::West => Orientation::North,
        }
    }

    pub fn left(&self) -> Orientation {
        match self {
            Orientation::North => Orientation::West,
            Orientation::West => Orientation::South,
            Orientation::South => Orientation::East,
            Orientation::East => Orientation::North,
        }
    }

    pub fn behind(&self) -> Orientation {
        match self {
            Orientation::North => Orientation::South,
            Orientation::East => Orientation::West,
            Orientation::South => Orientation::North,
            Orientation::West => Orientation::East,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_vector() {
        assert_eq!(Orientation::North.to_vector(), Vector::new(0, -1));
        assert_eq!(Orientation::East.to_vector(), Vector::new(1, 0));
        assert_eq!(Orientation::South.to_vector(), Vector::new(0, 1));
        assert_eq!(Orientation::West.to_vector(), Vector::new(-1, 0));
    }

    #[test]
    fn test_turn_right() {
        let mut dir = Orientation::North;
        dir.turn_right();
        assert!(matches!(dir, Orientation::East));
        dir.turn_right();
        assert!(matches!(dir, Orientation::South));
        dir.turn_right();
        assert!(matches!(dir, Orientation::West));
        dir.turn_right();
        assert!(matches!(dir, Orientation::North));
    }

    #[test]
    fn test_turn_left() {
        let mut dir = Orientation::North;
        dir.turn_left();
        assert!(matches!(dir, Orientation::West));
        dir.turn_left();
        assert!(matches!(dir, Orientation::South));
        dir.turn_left();
        assert!(matches!(dir, Orientation::East));
        dir.turn_left();
        assert!(matches!(dir, Orientation::North));
    }

    #[test]
    fn test_turn_180() {
        let mut dir = Orientation::North;
        dir.turn_180();
        assert!(matches!(dir, Orientation::South));

        let mut dir = Orientation::East;
        dir.turn_180();
        assert!(matches!(dir, Orientation::West));

        let mut dir = Orientation::South;
        dir.turn_180();
        assert!(matches!(dir, Orientation::North));

        let mut dir = Orientation::West;
        dir.turn_180();
        assert!(matches!(dir, Orientation::East));
    }

    #[test]
    fn test_right() {
        assert!(matches!(Orientation::North.right(), Orientation::East));
        assert!(matches!(Orientation::East.right(), Orientation::South));
        assert!(matches!(Orientation::South.right(), Orientation::West));
        assert!(matches!(Orientation::West.right(), Orientation::North));
    }

    #[test]
    fn test_left() {
        assert!(matches!(Orientation::North.left(), Orientation::West));
        assert!(matches!(Orientation::West.left(), Orientation::South));
        assert!(matches!(Orientation::South.left(), Orientation::East));
        assert!(matches!(Orientation::East.left(), Orientation::North));
    }

    #[test]
    fn test_behind() {
        assert!(matches!(Orientation::North.behind(), Orientation::South));
        assert!(matches!(Orientation::East.behind(), Orientation::West));
        assert!(matches!(Orientation::South.behind(), Orientation::North));
        assert!(matches!(Orientation::West.behind(), Orientation::East));
    }
}
