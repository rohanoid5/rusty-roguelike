use bracket_lib::{color::ColorPair, terminal::FontCharType};

#[derive(Clone, Debug, Copy, PartialEq)]
pub struct Render {
    pub color: ColorPair,
    pub glyph: FontCharType,
}

#[derive(Clone, Debug, Copy, PartialEq)]
pub struct Player;

#[derive(Clone, Debug, Copy, PartialEq)]
pub struct Enemy;
