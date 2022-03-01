use macroquad::prelude::*;

use crate::entities::Snake;
use crate::utils::Direction;

pub enum State {
    Playing,
    GameOver,
}

pub struct Game {
    score: i32,
    last_update: f64,
    state: State,
    snake: Snake,
}

impl Game {
    pub fn new() -> Self {
        Self {
            score: 0,
            last_update: 0.,
            state: State::Playing,
            snake: Snake::new(0, 0),
        }
    }

    pub async fn start(&mut self) {
        while matches!(self.state, State::Playing) {
            self.handle_keyboard_events();
            clear_background(BLACK);
            next_frame().await;
        }
    }

    fn handle_keyboard_events(&mut self) {
        if is_key_down(KeyCode::Escape) {
            panic!("Exit gracefully please!");
        }

        if is_key_down(KeyCode::Up) || is_key_down(KeyCode::W) {
            self.snake.update_direction(Direction::Up);
            dbg!("Keydown: Up");
            return;
        }

        if is_key_down(KeyCode::Down) || is_key_down(KeyCode::S) {
            self.snake.update_direction(Direction::Down);
            dbg!("Keydown: Down");
            return;
        }

        if is_key_down(KeyCode::Right) || is_key_down(KeyCode::D) {
            self.snake.update_direction(Direction::Right);
            dbg!("Keydown: Right");
            return;
        }

        if is_key_down(KeyCode::Left) || is_key_down(KeyCode::A) {
            self.snake.update_direction(Direction::Left);
            dbg!("Keydown: Left");
            return;
        }
    }
}
