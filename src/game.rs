use macroquad::prelude::*;

use crate::entities::Snake;

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
            clear_background(BLACK);
            next_frame().await;
        }
    }
}
