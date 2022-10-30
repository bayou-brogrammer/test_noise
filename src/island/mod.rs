mod biome;
mod bterm;
mod debug;
mod gradient;
mod noise_map;
mod noising;

mod island_prelude {
    pub use crate::prelude::*;

    pub use super::biome::*;
    pub use super::bterm::*;
    pub use super::debug::*;
    pub use super::gradient::*;
    pub use super::noise_map::*;
    pub use super::noising::*;

    pub type HeightMap = ndarray::Array2<f64>;
}
pub use island_prelude::*;

pub fn run_island_gen(app: &mut App) {
    app.add_plugin(bterm::BTermPlugin)
        .add_plugin(debug::DebugPlugin)
        .add_startup_system(setup)
        .add_system(render_noise)
        .run();
}

pub fn setup(mut commands: Commands) {
    let mut nm = InternalNoiseMap::default();
    nm.generate_maps();
    commands.insert_resource(nm);
}

pub fn render_noise(ctx: Res<BracketContext>, nm: Res<InternalNoiseMap>) {
    for x in 0..SCREEN_WIDTH {
        for y in 0..SCREEN_HEIGHT {
            let height = nm.height_map.get((x, y)).unwrap();
            let moisture = nm.moisture_map.get((x, y)).unwrap();

            let biome = get_biome_at(*height, *moisture);
            let (glyph, color) = get_biome_color(&biome);
            ctx.set(x as i32, y as i32, color, BLACK, glyph);
        }
    }
}
