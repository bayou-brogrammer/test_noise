use super::bracket_prelude::*;

// Screens
pub const SCREEN_WIDTH: i32 = 120;
pub const SCREEN_HEIGHT: i32 = 75;

fn setup_bterm() -> BTermBuilder {
    BTermBuilder::empty()
        .with_random_number_generator(true)
        .with_font("terminal8x8.png", 16, 16, (8.0, 8.0))
        .with_simple_console(0, SCREEN_WIDTH, SCREEN_HEIGHT)
        .with_background(true)
}

pub struct BTermPlugin;
impl Plugin for BTermPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(setup_bterm());
    }
}
