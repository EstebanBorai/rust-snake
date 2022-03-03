use super::{Coordinates, Coords, Direction};

#[derive(Clone, Debug)]
pub struct Block {
    coords: Coords,
}

impl Block {
    pub fn new(x: f32, y: f32) -> Block {
        Block {
            coords: Coords::new(x, y),
        }
    }

    pub fn get_coords(&self) -> Coords {
        self.coords
    }

    pub fn replicate_with_direction(&self, dir: &Direction, ammount: f32) -> Self {
        let Coords { x, y } = match dir {
            Direction::Up => self.coords.replicate_x(self.coords.get_y() - ammount),
            Direction::Down => self.coords.replicate_x(self.coords.get_y() + ammount),
            Direction::Right => self.coords.replicate_y(self.coords.get_x() + ammount),
            Direction::Left => self.coords.replicate_y(self.coords.get_x() - ammount),
        };

        Block::new(x, y)
    }
}

impl Coordinates for Block {
    fn get_x(&self) -> f32 {
        self.coords.x
    }

    fn get_y(&self) -> f32 {
        self.coords.y
    }
}
