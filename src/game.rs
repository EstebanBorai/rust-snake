use piston_window::types::Color;
use piston_window::*;
use rand::{thread_rng, Rng};

use crate::entities::Snake;
use crate::util::{draw_block, draw_rectangle, BlockCoordinates, Coords2D, Direction, BLACK, RED};

const FOOD_COLOR: Color = RED;
const BORDER_COLOR: Color = BLACK;
const GAMEOVER_COLOR: Color = RED;

const MOVING_PERIOD: f64 = 0.1;
const RESTART_TIME: f64 = 1.0;

pub struct Game {
    snake: Snake,
    food_exists: bool,
    food_position: BlockCoordinates,
    width: i32,
    height: i32,
    game_over: bool,
    waiting_time: f64,
}

impl Game {
    pub fn new(width: i32, height: i32) -> Game {
        Game {
            snake: Snake::new(2, 2),
            waiting_time: 0.0,
            food_exists: true,
            food_position: (6, 4),
            width,
            height,
            game_over: false,
        }
    }

    pub fn handle_key_pressed(&mut self, key: Key) {
        if self.game_over {
            return;
        }

        let dir = match key {
            Key::Up | Key::W => Some(Direction::Up),
            Key::Down | Key::S => Some(Direction::Down),
            Key::Left | Key::A => Some(Direction::Left),
            Key::Right | Key::D => Some(Direction::Right),
            _ => None,
        };

        if let Some(key) = dir {
            if key == self.snake.get_head_direction().opposite() {
                return;
            }

            self.update_snake(dir);
        }
    }

    pub fn draw(&self, con: &Context, g: &mut G2d) {
        self.snake.draw(con, g);

        if self.food_exists {
            draw_block(
                FOOD_COLOR,
                self.food_position.get_x(),
                self.food_position.get_y(),
                con,
                g,
            );
        }

        draw_rectangle(BORDER_COLOR, 0, 0, self.width, 1, con, g);
        draw_rectangle(BORDER_COLOR, 0, self.height - 1, self.width, 1, con, g);
        draw_rectangle(BORDER_COLOR, 0, 0, 1, self.height, con, g);
        draw_rectangle(BORDER_COLOR, self.width - 1, 0, 1, self.height, con, g);

        if self.game_over {
            draw_rectangle(GAMEOVER_COLOR, 0, 0, self.width, self.height, con, g);
        }
    }

    pub fn update(&mut self, delta_time: f64) {
        self.waiting_time += delta_time;

        if self.game_over {
            if self.waiting_time > RESTART_TIME {
                self.restart();
            }

            return;
        }

        if !self.food_exists {
            self.add_food();
        }

        if self.waiting_time > MOVING_PERIOD {
            self.update_snake(None);
        }
    }

    fn check_eating(&mut self) {
        let coords: BlockCoordinates = self.snake.get_head_position();

        if self.food_exists
            && self.food_position.get_x() == coords.get_x()
            && self.food_position.get_y() == coords.get_y()
        {
            self.food_exists = false;
            self.snake.restore_tail();
        }
    }

    fn is_snake_alive(&self, dir: Option<Direction>) -> bool {
        let coords: BlockCoordinates = self.snake.next_head(dir);

        if self.snake.overlap_tail(coords) {
            return false;
        }

        coords.get_x() > 0
            && coords.get_y() > 0
            && coords.get_x() < self.width - 1
            && coords.get_y() < self.height - 1
    }

    fn add_food(&mut self) {
        let mut rng = thread_rng();
        let mut new_x = rng.gen_range(1..self.width - 1);
        let mut new_y = rng.gen_range(1..self.width - 1);

        while self.snake.overlap_tail((new_x, new_y)) {
            new_x = rng.gen_range(1..self.width - 1);
            new_y = rng.gen_range(1..self.width - 1);
        }

        self.food_position = (new_x, new_y);
        self.food_exists = true;
    }

    fn update_snake(&mut self, dir: Option<Direction>) {
        if self.is_snake_alive(dir) {
            self.snake.handle_move(dir);
            self.check_eating();
        } else {
            self.game_over = true;
        }

        self.waiting_time = 0.0;
    }

    fn restart(&mut self) {
        self.snake = Snake::new(2, 2);
        self.waiting_time = 0.0;
        self.food_exists = true;
        self.food_position = (6, 4);
        self.game_over = false;
    }
}
