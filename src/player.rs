use crate::prelude::*;

pub struct Player {
    x: i32,
    y: i32,
}

impl Player {
    pub fn new() -> Self {
        Self {
            x: 0,
            y: 0,
        }
    }
    pub fn movement(&mut self, ctx: &mut BTerm) {

    }
}
