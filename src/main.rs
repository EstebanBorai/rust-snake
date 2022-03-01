mod entities;
mod game;
mod utils;

#[macroquad::main("Snake")]
async fn main() {
    let mut game = game::Game::new();

    game.start().await;
}
