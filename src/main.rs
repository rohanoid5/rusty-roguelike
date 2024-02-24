mod map;
mod state;

mod prelude {
    pub use bracket_lib::prelude::*;
    pub const SCREEN_HEIGHT: i32 = 50;
    pub const SCREEN_WIDTH: i32 = 80;
    pub use crate::map::*;
    pub use crate::state::*;
}

use crate::prelude::*;
const NUM_TILES: usize = (SCREEN_HEIGHT * SCREEN_WIDTH) as usize;

fn main() -> BError {
    println!("STARTING....");

    let context = BTermBuilder::simple80x50()
        .with_title("Rusty RogueLike")
        .with_fps_cap(30.0)
        .build()?;

    main_loop(context, State::new())
}
