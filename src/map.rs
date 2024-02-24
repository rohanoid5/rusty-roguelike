use bracket_lib::{
    color::{BLACK, RED4, YELLOW},
    random::RandomNumberGenerator,
    terminal::{to_cp437, BTerm},
};

use crate::{NUM_TILES, SCREEN_HEIGHT, SCREEN_WIDTH};

#[derive(Clone, Copy, PartialEq)]
pub enum TileType {
    Wall,
    Floor,
}

pub fn map_idx(x: i32, y: i32) -> usize {
    ((y * SCREEN_WIDTH) + x) as usize
}

pub struct Map {
    pub tiles: Vec<TileType>,
}

impl Map {
    pub fn new() -> Self {
        let mut tiles = vec![TileType::Floor; NUM_TILES];

        for idx in 0..NUM_TILES {
            let mut random = RandomNumberGenerator::new();
            let flag = random.range(0, 2);

            if flag == 0 {
                tiles[idx] = TileType::Floor
            } else {
                tiles[idx] = TileType::Wall
            }
        }

        Map { tiles }
    }

    pub fn render(&mut self, ctx: &mut BTerm) {
        for x in 0..SCREEN_WIDTH {
            for y in 0..SCREEN_HEIGHT {
                let idx = map_idx(x, y);

                match self.tiles[idx] {
                    TileType::Floor => ctx.set(x, y, YELLOW, BLACK, to_cp437('.')),
                    TileType::Wall => ctx.set(x, y, RED4, BLACK, to_cp437('#')),
                }
            }
        }
    }
}
