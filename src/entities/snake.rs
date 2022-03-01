use std::collections::LinkedList;

use crate::utils::{Block, Direction};

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
            body: LinkedList::new(),
            tail: None,
        }
    }
}
