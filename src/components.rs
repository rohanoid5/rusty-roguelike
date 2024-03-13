use bracket_lib::{
    color::ColorPair,
    terminal::{FontCharType, Point},
};
use legion::Entity;

#[derive(Clone, Debug, Copy, PartialEq)]
pub struct Render {
    pub color: ColorPair,
    pub glyph: FontCharType,
}

#[derive(Clone, Debug, Copy, PartialEq)]
pub struct Player;

#[derive(Clone, Debug, Copy, PartialEq)]
pub struct Enemy;

#[derive(Clone, Debug, Copy, PartialEq)]
pub struct MoveRandomly;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WantsToMove {
    pub entity: Entity,
    pub destination: Point,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Health {
    pub current: i32,
    pub max: i32,
}

#[derive(Clone, PartialEq)]
pub struct Name(pub String);
