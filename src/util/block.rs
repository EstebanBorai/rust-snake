use crate::util::Direction;

#[derive(Clone, Debug)]
pub struct Block {
    x: i32,
    y: i32,
}

pub type BlockCoordinates = (i32, i32);

impl Block {
    pub fn new(x: i32, y: i32) -> Block {
        Block { x, y }
    }

    pub fn get_coords(&self) -> BlockCoordinates {
        (self.x, self.y)
    }
}

pub trait BlockDirection {
    fn calc_direction(&self, dir: &Direction) -> (i32, i32);
}

impl BlockDirection for BlockCoordinates {
    fn calc_direction(&self, dir: &Direction) -> (i32, i32) {
        match dir {
            Direction::Up => (self.0, self.1 - 1),
            Direction::Down => (self.0, self.1 + 1),
            Direction::Right => (self.0 + 1, self.1),
            Direction::Left => (self.0 - 1, self.1),
        }
    }
}

pub trait Coords2D {
    fn get_x(&self) -> i32;
    fn get_y(&self) -> i32;
}

impl Coords2D for BlockCoordinates {
    fn get_x(&self) -> i32 {
        self.0
    }

    fn get_y(&self) -> i32 {
        self.1
    }
}

impl From<BlockCoordinates> for Block {
    fn from(coords: BlockCoordinates) -> Self {
        Block::new(coords.0, coords.1)
    }
}
