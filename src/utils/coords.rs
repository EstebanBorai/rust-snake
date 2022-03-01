#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Coords {
    pub(crate) x: i32,
    pub(crate) y: i32,
}

impl Coords {
    pub fn new(x: i32, y: i32) -> Self {
        Coords { x, y }
    }

    pub fn replicate_x(&self, y: i32) -> Self {
        Coords { x: self.x, y }
    }

    pub fn replicate_y(&self, x: i32) -> Self {
        Coords { x, y: self.y }
    }
}

pub trait Coordinates {
    fn get_x(&self) -> i32;
    fn get_y(&self) -> i32;
}

impl Coordinates for Coords {
    fn get_x(&self) -> i32 {
        self.x
    }

    fn get_y(&self) -> i32 {
        self.y
    }
}
