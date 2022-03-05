use macroquad::prelude::*;

use crate::entities::{Food, Snake};
use crate::utils::{Coordinates, Direction};

pub const BLOCK_SIZE: f32 = 16.;

pub enum State {
    Playing,
    GameOver,
}

pub struct Game {
    food: Food,
    score: i32,
    screen_height: f32,
    screen_width: f32,
    state: State,
    snake: Snake,
    last_update: f64,
}

impl Game {
    pub fn new() -> Self {
        let screen_height = screen_height();
        let screen_width = screen_width();
        let food = Food::new(screen_width, screen_height);
        let snake = Snake::new(screen_width, screen_height);

        Self {
            food,
            score: 0,
            screen_height,
            screen_width,
            state: State::Playing,
            snake,
            last_update: get_time(),
        }
    }

    pub async fn start(&mut self) {
        while matches!(self.state, State::Playing) {
            self.handle_keyboard_events();

            if (get_time() - self.last_update) > self.snake.speed() as f64 {
                self.last_update = get_time();
                self.snake.forward();
                self.check_collision();
            }

            clear_background(BLACK);

            // Draw Score
            draw_text(
                format!("SCORE: {}", self.score).as_str(),
                24.,
                24.,
                24.,
                WHITE,
            );

            // Draw the food
            draw_rectangle(
                self.food.get_x(),
                self.food.get_y(),
                BLOCK_SIZE,
                BLOCK_SIZE,
                RED,
            );

            // Draw the snake
            for block in self.snake.blocks() {
                let coords = block.get_coords();

                draw_rectangle(coords.x, coords.y, BLOCK_SIZE, BLOCK_SIZE, GREEN);
            }

            next_frame().await;
        }

        while matches!(self.state, State::GameOver) {
            if is_key_down(KeyCode::Escape) {
                panic!("Exit gracefully please!");
            }

            let font_size = 32;
            let text = format!("Game Over! Your score was: {}", self.score);
            let text_dimensions = measure_text(text.as_str(), None, font_size, 1.);

            // Draw Score
            draw_text(
                text.as_str(),
                (self.screen_width - text_dimensions.width) / 2.,
                self.screen_height / 2.,
                font_size as f32,
                WHITE,
            );
            next_frame().await;
        }
    }

    fn handle_keyboard_events(&mut self) {
        if is_key_down(KeyCode::Escape) {
            panic!("Exit gracefully please!");
        }

        if is_key_down(KeyCode::Up) || is_key_down(KeyCode::W) {
            self.snake.update_direction(Direction::Up);
            return;
        }

        if is_key_down(KeyCode::Down) || is_key_down(KeyCode::S) {
            self.snake.update_direction(Direction::Down);
            return;
        }

        if is_key_down(KeyCode::Right) || is_key_down(KeyCode::D) {
            self.snake.update_direction(Direction::Right);
            return;
        }

        if is_key_down(KeyCode::Left) || is_key_down(KeyCode::A) {
            self.snake.update_direction(Direction::Left);
            return;
        }
    }

    fn check_collision(&mut self) {
        let error = BLOCK_SIZE / 0.95;
        let snake_head_coords = self.snake.get_head_block();
        let (snake_head_x, snake_head_y) = (snake_head_coords.get_x(), snake_head_coords.get_y());
        let (food_x, food_y) = (self.food.get_x(), self.food.get_y());

        if snake_head_x <= 16.
            || snake_head_x >= self.screen_width - 16.
            || snake_head_y <= 16.
            || snake_head_y >= self.screen_height - 16.
        {
            self.state = State::GameOver;
        }

        if (snake_head_x - food_x).abs() <= error && (snake_head_y - food_y).abs() <= error {
            self.snake.eat();
            self.food.eat();
            self.score += 1;
        }
    }
}
