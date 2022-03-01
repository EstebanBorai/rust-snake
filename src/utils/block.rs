use super::{Coordinates, Coords, Direction};

#[derive(Clone, Debug)]
pub struct Block {
    coords: Coords,
}

impl Block {
    pub fn new(x: i32, y: i32) -> Block {
        Block {
            coords: Coords::new(x, y),
        }
    }

    pub fn get_coords(&self) -> Coords {
        self.coords
    }

    pub fn replicate_with_direction(&self, dir: &Direction) -> Self {
        let Coords { x, y } = match dir {
            Direction::Up => self.coords.replicate_x(self.coords.get_y() - 1),
            Direction::Down => self.coords.replicate_x(self.coords.get_y() + 1),
            Direction::Right => self.coords.replicate_y(self.coords.get_x() + 1),
            Direction::Left => self.coords.replicate_y(self.coords.get_x() - 1),
        };

        Block::new(x, y)
    }
}
