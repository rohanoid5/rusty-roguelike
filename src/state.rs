use crate::{Camera, Map, MapBuilder, Player};
use bracket_lib::{
    random::RandomNumberGenerator,
    terminal::{BTerm, GameState},
};

pub struct State {
    map: Map,
    player: Player,
    camera: Camera,
}

impl State {
    pub fn new() -> Self {
        let mut range = RandomNumberGenerator::new();
        let map_builder = MapBuilder::new(&mut range);

        State {
            map: map_builder.map,
            player: Player::new(map_builder.player_start),
            camera: Camera::new(map_builder.player_start),
        }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        self.player.update(ctx, &self.map, &mut self.camera);
        self.map.render(ctx, &self.camera);
        self.player.render(ctx, &self.camera);
    }
}
