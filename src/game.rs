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
            state: State::GameOver,
            snake,
        }
    }

    pub async fn start(&mut self) {
        while matches!(self.state, State::Playing) {
            self.screen_height = screen_height();
            self.screen_width = screen_width();
            self.handle_keyboard_events();

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

            self.found_food();
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

    fn found_food(&mut self) {
        let head = self.snake.get_head_block();
        let error = BLOCK_SIZE / 0.95;

        if (head.get_x() - self.food.get_x()).abs() <= error
            && (head.get_y() - self.food.get_y()).abs() <= error
        {
            self.snake.eat();
            self.food.eat();
            self.score += 1;
        }
    }

    fn handle_keyboard_events(&mut self) {
        if is_key_down(KeyCode::Escape) {
            panic!("Exit gracefully please!");
        }

        if is_key_down(KeyCode::Up) || is_key_down(KeyCode::W) {
            self.snake.update_direction(Direction::Up);
            self.snake.forward();
            dbg!("Keydown: Up");
            return;
        }

        if is_key_down(KeyCode::Down) || is_key_down(KeyCode::S) {
            self.snake.update_direction(Direction::Down);
            self.snake.forward();
            dbg!("Keydown: Down");
            return;
        }

        if is_key_down(KeyCode::Right) || is_key_down(KeyCode::D) {
            self.snake.update_direction(Direction::Right);
            self.snake.forward();
            dbg!("Keydown: Right");
            return;
        }

        if is_key_down(KeyCode::Left) || is_key_down(KeyCode::A) {
            self.snake.update_direction(Direction::Left);
            self.snake.forward();
            dbg!("Keydown: Left");
            return;
        }

        self.snake.forward();
    }
}
