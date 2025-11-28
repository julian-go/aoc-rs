pub use super::orientation::Orientation;
pub use super::vector::Vector;

#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
pub struct Pose {
    pub position: Vector,
    pub orientation: Orientation,
}

impl Pose {
    pub fn new(x: i64, y: i64, orientation: Orientation) -> Self {
        Self {
            position: Vector::new(x, y),
            orientation,
        }
    }

    pub fn new_xy(x: i64, y: i64) -> Self {
        Self {
            position: Vector::new(x, y),
            orientation: Orientation::North,
        }
    }

    pub fn adjacent(&self) -> [Pose; 4] {
        [
            Pose {
                position: self.position + self.orientation.to_vector(),
                orientation: self.orientation,
            },
            Pose {
                position: self.position + self.orientation.right().to_vector(),
                orientation: self.orientation,
            },
            Pose {
                position: self.position + self.orientation.behind().to_vector(),
                orientation: self.orientation,
            },
            Pose {
                position: self.position + self.orientation.left().to_vector(),
                orientation: self.orientation,
            },
        ]
    }

    pub fn adjacent_vec(&self) -> [Vector; 4] {
        [
            self.position + self.orientation.to_vector(),
            self.position + self.orientation.right().to_vector(),
            self.position + self.orientation.behind().to_vector(),
            self.position + self.orientation.left().to_vector(),
        ]
    }

    pub fn move_forward(&mut self, distance: i64) {
        let movement = self.orientation.to_vector() * distance;
        self.position += movement;
    }

    pub fn move_backward(&mut self, distance: i64) {
        let movement = self.orientation.to_vector() * distance;
        self.position -= movement;
    }

    pub fn move_left(&mut self, distance: i64) {
        let movement = self.orientation.left().to_vector() * distance;
        self.position += movement;
    }

    pub fn move_right(&mut self, distance: i64) {
        let movement = self.orientation.right().to_vector() * distance;
        self.position += movement;
    }
}

impl From<Pose> for (usize, usize) {
    fn from(v: Pose) -> (usize, usize) {
        v.position.into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_adjacent() {
        let pose = Pose::new(5, 5, Orientation::North);
        let adjacent = pose.adjacent_vec();

        assert_eq!(adjacent[0], Vector::new(5, 4)); // forward (north)
        assert_eq!(adjacent[1], Vector::new(6, 5)); // right (east)
        assert_eq!(adjacent[2], Vector::new(5, 6)); // behind (south)
        assert_eq!(adjacent[3], Vector::new(4, 5)); // left (west)
    }

    #[test]
    fn test_move_forward() {
        let mut pose = Pose::new(0, 0, Orientation::North);
        pose.move_forward(3);
        assert_eq!(pose.position, Vector::new(0, -3));

        let mut pose = Pose::new(0, 0, Orientation::East);
        pose.move_forward(5);
        assert_eq!(pose.position, Vector::new(5, 0));
    }

    #[test]
    fn test_move_backward() {
        let mut pose = Pose::new(0, 0, Orientation::North);
        pose.move_backward(3);
        assert_eq!(pose.position, Vector::new(0, 3));

        let mut pose = Pose::new(0, 0, Orientation::West);
        pose.move_backward(2);
        assert_eq!(pose.position, Vector::new(2, 0));
    }

    #[test]
    fn test_move_left() {
        let mut pose = Pose::new(0, 0, Orientation::North);
        pose.move_left(4);
        assert_eq!(pose.position, Vector::new(-4, 0));

        let mut pose = Pose::new(0, 0, Orientation::East);
        pose.move_left(3);
        assert_eq!(pose.position, Vector::new(0, -3));
    }

    #[test]
    fn test_move_right() {
        let mut pose = Pose::new(0, 0, Orientation::North);
        pose.move_right(2);
        assert_eq!(pose.position, Vector::new(2, 0));

        let mut pose = Pose::new(0, 0, Orientation::South);
        pose.move_right(5);
        assert_eq!(pose.position, Vector::new(-5, 0));
    }

    #[test]
    fn test_complex_movement() {
        let mut pose = Pose::new(0, 0, Orientation::North);
        pose.move_forward(5);
        pose.orientation.turn_right();
        pose.move_forward(3);
        pose.orientation.turn_right();
        pose.move_backward(2);
        pose.move_left(1);

        assert_eq!(pose.position, Vector::new(4, -7));
    }
}
