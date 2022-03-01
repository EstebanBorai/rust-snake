use std::collections::LinkedList;

use crate::utils::{Block, Coords, Direction};

pub struct Snake {
    direction: Direction,
    body: LinkedList<Block>,
    tail: Option<Block>,
}

impl Snake {
    pub fn new(x: i32, y: i32) -> Self {
        let mut body: LinkedList<Block> = LinkedList::new();

        body.push_back(Block::new(x + 2, y));
        body.push_back(Block::new(x, y));

        Self {
            direction: Direction::Up,
            body,
            tail: None,
        }
    }

    pub fn update_direction(&mut self, dir: Direction) {
        let head_block = self.body.front().unwrap();
        let next_head_block = head_block.replicate_with_direction(&dir);

        self.body.push_front(next_head_block);

        let removed = self.body.pop_back();

        self.tail = removed;
        self.direction = dir;
    }

    pub fn blocks(&self) -> LinkedList<Block> {
        self.body.clone()
    }

    fn head_coords(&self) -> Coords {
        self.body.front().unwrap().get_coords()
    }
}
