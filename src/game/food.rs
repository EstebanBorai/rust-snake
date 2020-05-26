use opengl_graphics::GlGraphics;
use piston::input::*;
use crate::game::Snake;
use crate::graphics;

extern crate rand;

pub struct Food {
  pub x: u32,
  pub y: u32,
}

impl Food {
  pub fn new() -> Self {
    Food {
      x: 1,
      y: 1
    }
  }

  pub fn update(&mut self, s: &Snake) -> bool {
    let front = s.snake_parts.front().unwrap();

    if front.0 == self.x && front.1 == self.y {
      true
    } else {
      false
    }
  }

  pub fn render(&mut self, gl: &mut GlGraphics, args: &RenderArgs, width: u32) {
    use graphics;

    const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];

    let x = self.x * width;
    let y = self.y * width;

    let square = graphics::rectangle::square(x as f64, y as f64, width as f64);

    gl.draw(args.viewport(), |c, gl| {
      let transform = c.transform;

      graphics::rectangle(WHITE, square, transform, gl);
    });
  }
}
