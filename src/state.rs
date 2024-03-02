use crate::{
    map_idx, Camera, Map, MapBuilder, Obstacle, Player, TileType, DISPLAY_HEIGHT, DISPLAY_WIDTH,
    OBSTACLE_NUMBER, SCREEN_HEIGHT, SCREEN_WIDTH,
};
use bracket_lib::{
    random::RandomNumberGenerator,
    terminal::{BTerm, GameState, Point},
};

pub struct State {
    map: Map,
    player: Player,
    camera: Camera,
    score: usize,
    obstacles: Vec<Obstacle>,
}

impl State {
    pub fn new() -> Self {
        let mut range = RandomNumberGenerator::new();
        let map_builder = MapBuilder::new(&mut range);
        let mut obstacles: Vec<Obstacle> = Vec::new();

        while (obstacles.len() as i32) < OBSTACLE_NUMBER {
            let x = range.range(0, SCREEN_WIDTH);
            let y = range.range(0, SCREEN_HEIGHT);

            if map_builder.map.tiles[map_idx(x, y)] == TileType::Floor {
                let obstacle = Obstacle::new(Point { x, y });
                obstacles.push(obstacle)
            }
        }

        State {
            map: map_builder.map,
            player: Player::new(map_builder.player_start),
            camera: Camera::new(map_builder.player_start),
            score: 0,
            obstacles,
        }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        self.player
            .update(ctx, &self.map, &mut self.camera, &mut self.score);
        self.map.render(ctx, &self.camera);
        self.player.render(ctx, &self.camera);

        for obstacle in &self.obstacles {
            obstacle.render(ctx, &mut self.camera)
        }

        ctx.set_active_console(1);
        ctx.print(0, 0, format!("Score: {}", self.score));
    }
}
