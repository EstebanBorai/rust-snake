use std::iter::FromIterator;
use std::collections::LinkedList;
use crate::game::Direction;
use crate::piston::input::*;
use crate::graphics;
use crate::opengl_graphics::{OpenGL, GlGraphics};

pub struct Snake {
  gl: GlGraphics,
  pub snake_parts: LinkedList<SnakePiece>,
  pub width: u32,
  pub dir: Direction,
}

#[derive(Clone)]
pub struct SnakePiece(pub u32, pub u32);

impl Snake {
  pub fn new(gl: OpenGL, cols: u32, rows: u32, square_width: u32) -> Self {
    Snake {
      gl: GlGraphics::new(gl),
      snake_parts: LinkedList::from_iter((vec![SnakePiece(cols / 2, rows / 2)]).into_iter()),
      width: square_width,
      dir: Direction::Down,
    }
  }

  pub fn render(&mut self, args: &RenderArgs) {
    use graphics;

    const DARK_GREEN: [f32; 4] = [0.0, 0.5, 0.0, 1.0];

    let squares: Vec<graphics::types::Rectangle> = self.snake_parts
      .iter()
      .map(|p| SnakePiece(p.0 * self.width, p.1 * self.width))
      .map(|p| graphics::rectangle::square(p.0 as f64, p.1 as f64, self.width as f64))
      .collect();

    self.gl.draw(args.viewport(), |c, gl| {
      let transform = c.transform;

      squares
        .into_iter()
        .for_each(|square| graphics::rectangle(DARK_GREEN, square, transform, gl));
    })
  }

  pub fn update(&mut self, just_eaten: bool, cols: u32, rows: u32) -> bool {
    let mut new_front: SnakePiece = (*self.snake_parts.front().expect("No front of snake found!")).clone();

    if (self.dir == Direction::Up && new_front.1 == 0)
      || (self.dir == Direction::Left && new_front.0 == 0)
      || (self.dir == Direction::Down && new_front.1 == rows -1)
      || (self.dir == Direction::Right && new_front.0 == cols -1)
    {
      return false;
    }

    match self.dir {
      Direction::Up => new_front.1 -= 1,
      Direction::Down => new_front.1 += 1,
      Direction::Left => new_front.0 -= 1,
      Direction::Right => new_front.0 += 1,
    }

    if !just_eaten {
      self.snake_parts.pop_back();
    }

    if self.is_collide(new_front.0, new_front.1) {
      return false;
    }

    self.snake_parts.push_front(new_front);
    true
  }

  pub fn is_collide(&self, x: u32, y: u32) -> bool {
    self.snake_parts.iter().any(|p| x == p.0 && y == p.1)
  }
}
