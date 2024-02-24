use crate::{Map, Player};
use bracket_lib::terminal::{BTerm, GameState, VirtualKeyCode};

pub struct State {
    map: Map,
    player: Player,
}

impl State {
    pub fn new() -> Self {
        State {
            map: Map::new(),
            player: Player::new(),
        }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        self.map.render(ctx);

        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::Up => self.player.move_up(),
                VirtualKeyCode::Down => self.player.move_down(),
                VirtualKeyCode::Left => self.player.move_left(),
                VirtualKeyCode::Right => self.player.move_right(),
                _ => {}
            }
        }

        self.player.render(ctx);
    }
}
