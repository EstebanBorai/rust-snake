extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
use opengl_graphics::{GlGraphics, OpenGL};

extern crate rand;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow;

mod game;

fn main() {
    let opengl = OpenGL::V3_2;

    const COLS: u32 = 30;
    const ROWS: u32 = 20;
    const SQUARE_WIDTH: u32 = 20;

    let WIDTH = COLS * SQUARE_WIDTH;
    let HEIGHT = ROWS * SQUARE_WIDTH;

    let mut window: GlutinWindow = WindowSettings::new("🐍 Rust Snake", [WIDTH, HEIGHT])
      .opengl(opengl)
      .exit_on_esc(true)
      .build()
      .unwrap();

    let mut game = game::Game::new(opengl, COLS, ROWS, SQUARE_WIDTH);
    game.start(opengl, &mut window);
}
