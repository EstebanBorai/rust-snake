use crate::utils::{Coordinates, Coords};

pub struct Food {
    screen_width: f32,
    screen_height: f32,
    coords: Coords,
}

impl Food {
    pub fn new(screen_width: f32, screen_height: f32) -> Self {
        let coords = Coords::random(screen_width, screen_height);

        Food {
            screen_width,
            screen_height,
            coords,
        }
    }

    pub fn random_respawn(&mut self) {
        self.coords = Coords::random(self.screen_width - 16., self.screen_height - 16.);
    }
}

impl Coordinates for Food {
    fn get_x(&self) -> f32 {
        self.coords.x
    }

    fn get_y(&self) -> f32 {
        self.coords.y
    }
}
