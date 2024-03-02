use bracket_lib::{
    color::{BLACK, WHITE},
    terminal::{to_cp437, BTerm, Point},
};

use crate::Camera;

pub struct Obstacle {
    pub position: Point,
}

impl Obstacle {
    pub fn new(point: Point) -> Self {
        Obstacle { position: point }
    }

    pub fn render(&self, ctx: &mut BTerm, camera: &mut Camera) {
        ctx.set_active_console(1);
        ctx.set(
            self.position.x - camera.left_x,
            self.position.y - camera.top_y,
            WHITE,
            BLACK,
            to_cp437('E'),
        );
    }
}
