
use ggez;

mod state;
mod player;

use crate::state::MainState;

pub const WINDOW_RESOLUTION: (f32, f32) = (1024., 768.);

fn main() -> ggez::GameResult {
    let (mut ctx, mut event_loop) = ggez::ContextBuilder::new("grid-game", "ggez")
        .window_setup(
            ggez::conf::WindowSetup::default()
                .title("Grid-Game")
        )
        .window_mode(
            ggez::conf::WindowMode::default()
                .dimensions(WINDOW_RESOLUTION.0, WINDOW_RESOLUTION.1)
                .resizable(false)
        )
        .build()?;
    let state = &mut MainState::new()?;
    ggez::event::run(&mut ctx, &mut event_loop, state)
}
