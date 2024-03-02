use crate::{build_scheduler, spawn_monster, spawn_player, Camera, MapBuilder};
use bracket_lib::{
    random::RandomNumberGenerator,
    terminal::{render_draw_buffer, BTerm, GameState},
};
use legion::{Resources, Schedule, World};

pub struct State {
    ecs: World,
    resources: Resources,
    systems: Schedule,
}

impl State {
    pub fn new() -> Self {
        let mut ecs = World::default();
        let mut resources = Resources::default();
        let mut range = RandomNumberGenerator::new();
        let map_builder = MapBuilder::new(&mut range);

        spawn_player(&mut ecs, map_builder.player_start);
        map_builder
            .rooms
            .iter()
            .skip(1)
            .map(|room| room.center())
            .for_each(|pos| spawn_monster(&mut ecs, &mut range, pos));
        resources.insert(map_builder.map);
        resources.insert(Camera::new(map_builder.player_start));

        Self {
            ecs,
            resources,
            systems: build_scheduler(),
        }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.set_active_console(0);
        ctx.cls();
        ctx.set_active_console(1);
        ctx.cls();
        self.resources.insert(ctx.key);
        self.systems.execute(&mut self.ecs, &mut self.resources);
        render_draw_buffer(ctx).expect("Render error");
    }
}
