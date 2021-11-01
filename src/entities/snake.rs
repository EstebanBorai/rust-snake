use crate::util::{
    draw_block, Block, BlockCoordinates, BlockDirection, Coords2D, Direction, GREEN,
};
use piston_window::types::Color;
use piston_window::{Context, G2d};
use std::collections::LinkedList;

type SnakeBody = LinkedList<Block>;

const SNAKE_COLOR: Color = GREEN;

pub struct Snake {
    direction: Direction,
    body: LinkedList<Block>,
    tail: Option<Block>,
}

impl Snake {
    pub fn new(x: i32, y: i32) -> Self {
        let mut body: SnakeBody = LinkedList::new();

        body.push_back(Block::new(x + 2, y));
        body.push_back(Block::new(x, y));

        Snake {
            direction: Direction::Right,
            body,
            tail: None,
        }
    }

    pub fn draw(&self, con: &Context, g: &mut G2d) {
        for block in &self.body {
            let (x, y) = block.get_coords();

            draw_block(SNAKE_COLOR, x, y, con, g)
        }
    }

    pub fn get_head_position(&self) -> BlockCoordinates {
        self.body.front().unwrap().get_coords()
    }

    pub fn handle_move(&mut self, dir: Option<Direction>) {
        match dir {
            Some(next_direction) => self.direction = next_direction,
            None => (),
        }

        let current_coords: BlockCoordinates = self.get_head_position();

        let next_position: Block = match self.direction {
            Direction::Up => Block::from(current_coords.calc_direction(&Direction::Up)),
            Direction::Down => Block::from(current_coords.calc_direction(&Direction::Down)),
            Direction::Right => Block::from(current_coords.calc_direction(&Direction::Right)),
            Direction::Left => Block::from(current_coords.calc_direction(&Direction::Left)),
        };

        self.body.push_front(next_position);
        let removed = self.body.pop_back().unwrap();
        self.tail = Some(removed);
    }

    pub fn get_head_direction(&self) -> Direction {
        self.direction
    }

    pub fn next_head(&self, dir: Option<Direction>) -> BlockCoordinates {
        let current_coords: BlockCoordinates = self.get_head_position();

        let mut moving_dir = self.direction;

        match dir {
            Some(d) => moving_dir = d,
            None => {}
        };

        match moving_dir {
            Direction::Up => current_coords.calc_direction(&Direction::Up),
            Direction::Down => current_coords.calc_direction(&Direction::Down),
            Direction::Right => current_coords.calc_direction(&Direction::Right),
            Direction::Left => current_coords.calc_direction(&Direction::Left),
        }
    }

    pub fn restore_tail(&mut self) {
        let blk = self.tail.clone().unwrap();

        self.body.push_back(blk);
    }

    pub fn overlap_tail(&self, coords: BlockCoordinates) -> bool {
        let mut ch = 0;

        for block in &self.body {
            let (x, y) = block.get_coords();

            if coords.get_x() == x && coords.get_y() == y {
                return true;
            }

            ch += 1;

            if ch == self.body.len() - 1 {
                break;
            }
        }

        return false;
    }
}
