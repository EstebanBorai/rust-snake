use rand;
use piston_window::*;
use piston_window::types::Color;

mod entities;
mod game;
mod util;

fn main() {
  let (width, height) = (20, 20);

  let mut window: PistonWindow = WindowSettings::new(
    "Rust Snake Game",
    [util::to_coord_u32(width), util::to_coord_u32(height)],
  ).exit_on_esc(true)
   .build()
   .unwrap();

  let mut game = game::Game::new(width, height);

  while let Some(event) = window.next() {
    if let Some(Button::Keyboard(key)) = event.press_args() {
      game.handle_key_pressed(key);
    }

    window.draw_2d(&event, |ctx, gph, _d| {
      clear(util::GRAY, gph);
      game.draw(&ctx, gph);
    });

    event.update(|arg| {
      game.update(arg.dt)
    });
  }
}
