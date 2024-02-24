use bracket_lib::{
    color::{BLACK, GREEN},
    terminal::{to_cp437, BTerm},
};

use crate::{SCREEN_HEIGHT, SCREEN_WIDTH};

pub struct Player {
    x: i32,
    y: i32,
}

impl Player {
    pub fn new() -> Self {
        Self { x: 0, y: 0 }
    }

    pub fn render(&self, ctx: &mut BTerm) {
        ctx.set(self.x, self.y, GREEN, BLACK, to_cp437('^'));
    }

    pub fn move_right(&mut self) {
        if self.x < SCREEN_WIDTH - 1 {
            self.x += 1;
        }
    }

    pub fn move_left(&mut self) {
        if self.x > 0 {
            self.x -= 1;
        }
    }

    pub fn move_up(&mut self) {
        if self.y > 0 {
            self.y -= 1;
        }
    }

    pub fn move_down(&mut self) {
        if self.y < SCREEN_HEIGHT - 1 {
            self.y += 1;
        }
    }
}
