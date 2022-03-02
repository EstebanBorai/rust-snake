#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Coords {
    pub(crate) x: f32,
    pub(crate) y: f32,
}

impl Coords {
    pub fn new(x: f32, y: f32) -> Self {
        Coords { x, y }
    }

    pub fn replicate_x(&self, y: f32) -> Self {
        Coords { x: self.x, y }
    }

    pub fn replicate_y(&self, x: f32) -> Self {
        Coords { x, y: self.y }
    }
}

pub trait Coordinates {
    fn get_x(&self) -> f32;
    fn get_y(&self) -> f32;
}

impl Coordinates for Coords {
    fn get_x(&self) -> f32 {
        self.x
    }

    fn get_y(&self) -> f32 {
        self.y
    }
}
