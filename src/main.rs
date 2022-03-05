mod entities;
mod game;
mod utils;

use macroquad::prelude::*;

fn conf() -> Conf {
    Conf {
        window_title: String::from("Snake"),
        window_width: 512,
        window_height: 512,
        fullscreen: false,
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(conf)]
async fn main() {
    let mut game = game::Game::new();

    game.start().await;
}
