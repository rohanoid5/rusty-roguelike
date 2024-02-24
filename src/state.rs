use crate::Map;
use bracket_lib::terminal::{BTerm, GameState};

pub struct State {
    map: Map,
}

impl State {
    pub fn new() -> Self {
        State { map: Map::new() }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        self.map.render(ctx);
    }
}
