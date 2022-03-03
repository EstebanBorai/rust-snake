use std::collections::LinkedList;

use crate::utils::{Block, Direction};

pub struct Snake {
    direction: Direction,
    body: LinkedList<Block>,
    tail: Option<Block>,
}

impl Snake {
    pub fn new(screen_width: f32, screen_height: f32) -> Self {
        let mut body: LinkedList<Block> = LinkedList::new();

        body.push_back(Block::new(screen_width / 2., screen_height / 2.));

        Self {
            direction: Direction::Up,
            body,
            tail: None,
        }
    }

    pub fn update_direction(&mut self, dir: Direction) {
        let head_block = self.get_head_block();
        let next_head_block = head_block.replicate_with_direction(&dir, 4.);

        self.body.push_front(next_head_block);

        let removed = self.body.pop_back();

        self.tail = removed;
        self.direction = dir;
    }

    pub fn eat(&mut self) {
        let tail = self.tail.clone().unwrap();

        self.body.push_back(tail);
    }

    pub fn get_head_block(&self) -> Block {
        self.body.front().unwrap().to_owned()
    }

    pub fn blocks(&self) -> LinkedList<Block> {
        self.body.clone()
    }
}
