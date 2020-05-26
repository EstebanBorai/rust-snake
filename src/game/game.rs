use crate::game::{Snake, Food};
extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use crate::rand::{thread_rng, Rng};
use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow;
use opengl_graphics::{GlGraphics, OpenGL};

pub struct Game {
  gl: GlGraphics,
  rows: u32,
  cols: u32,
  snake: Snake,
  just_eaten: bool,
  square_width: u32,
  food: Food,
  score: u32,
}

#[derive(Clone, PartialEq)]
pub enum Direction {
  Up,
  Down,
  Left,
  Right
}

impl Game {
  pub fn new(gl: OpenGL, cols: u32, rows: u32, square_width: u32) -> Self {
    Game {
      gl: GlGraphics::new(gl),
      rows: 20,
      cols: 30,
      square_width: 20,
      just_eaten: false,
      food: Food::new(),
      score: 0,
      snake: Snake::new(gl, cols, rows, square_width),
    }
  }

  pub fn start(&mut self, opengl: OpenGL, window: &mut GlutinWindow) {
    let mut events = Events::new(EventSettings::new()).ups(10);
    while let Some(e) = events.next(window) {
        if let Some(r) = e.render_args() {
          self.render(&r);
        }

        if let Some(u) = e.update_args() {
            if !self.update(&u) {
                break;
            }
        }

        if let Some(k) = e.button_args() {
            if k.state == ButtonState::Press {
              self.pressed(&k.button);
            }
        }
    }
    println!("Congratulations, your score was: {}", self.score);
  }

  fn render(&mut self, args: &RenderArgs) {
    use graphics;

    const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

    self.gl.draw(args.viewport(), |c, gl| {
      graphics::clear(BLACK, gl);
    });

    self.snake.render(args);
    self.food.render(&mut self.gl, args, self.square_width);
  }

  fn update(&mut self, args: &UpdateArgs) -> bool {
    if !self.snake.update(self.just_eaten, self.cols, self.rows) {
      return false;
    }

    if self.just_eaten {
      self.score += 1;
      self.just_eaten = false;
    }

    self.just_eaten = self.food.update(&self.snake);

    if self.just_eaten {

      // try my luck
      let mut r = thread_rng();

      loop {
        let new_x = r.gen_range(0, self.cols);
        let new_y = r.gen_range(0, self.rows);

        if !self.snake.is_collide(new_x, new_y) {
          self.food = Food {
            x: new_x,
            y: new_y
          };
          break;
        }
      }
    }

    true
  }

  fn pressed(&mut self, btn: &Button) {
    let last_direction = self.snake.dir.clone();

    self.snake.dir = match btn {
      &Button::Keyboard(Key::Up) if last_direction != Direction::Down => Direction::Up,
      &Button::Keyboard(Key::Right) if last_direction != Direction::Left => Direction::Right,
      &Button::Keyboard(Key::Down) if last_direction != Direction::Up => Direction::Down,
      &Button::Keyboard(Key::Left) if last_direction != Direction::Right => Direction::Left,
      _ => last_direction,
    }
  }
}
