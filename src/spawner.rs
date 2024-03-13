use bracket_lib::{
    color::{ColorPair, BLACK, WHITE},
    random::RandomNumberGenerator,
    terminal::{to_cp437, FontCharType, Point},
};
use legion::World;

use crate::{Enemy, Health, MoveRandomly, Name, Player, Render};

fn goblin() -> (i32, String, FontCharType) {
    (1, "Goblin".to_string(), to_cp437('g'))
}

fn orc() -> (i32, String, FontCharType) {
    (2, "Orc".to_string(), to_cp437('o'))
}

pub fn spawn_player(ecs: &mut World, position: Point) {
    ecs.push((
        Player,
        position,
        Render {
            color: ColorPair::new(WHITE, BLACK),
            glyph: to_cp437('@'),
        },
        Health {
            current: 20,
            max: 20,
        },
    ));
}

pub fn spawn_monster(ecs: &mut World, range: &mut RandomNumberGenerator, position: Point) {
    let (hp, name, glyph) = match range.roll_dice(1, 10) {
        1..=8 => goblin(),
        _ => orc(),
    };

    ecs.push((
        Enemy,
        position,
        Render {
            color: ColorPair::new(WHITE, BLACK),
            glyph,
        },
        MoveRandomly {},
        Health {
            current: hp,
            max: hp,
        },
        Name(name),
    ));
}
