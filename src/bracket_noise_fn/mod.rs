mod bterm;
mod map;
mod noise;

mod bracket_prelude {
    pub use crate::prelude::*;
    pub use bracket_bevy::prelude::*;
    pub use bracket_noise::prelude::*;
    pub use sark_grids::*;

    pub use super::bterm::*;
    pub use super::map::*;
    pub use super::noise::*;

    pub const SCREEN_WIDTH: i32 = 120;
    pub const SCREEN_HEIGHT: i32 = 75;
}
use bevy_inspector_egui::prelude::*;
use bevy_inspector_egui::reflect::ReflectedUI;
pub use bracket_prelude::*;

#[derive(Inspectable, Default, Debug)]
struct Data {
    // it works for custom reflect types
    custom: ReflectedUI<NoiseSettings>,
}

pub fn run_bracket_instance(app: &mut App) {
    app.add_plugins(DefaultPlugins)
        .add_plugin(InspectorPlugin::<Data>::new())
        .add_plugin(bterm::BTermPlugin)
        .add_startup_system(setup)
        .add_system(render)
        .add_system(regen)
        .register_type::<NoiseSettings>()
        .register_type::<DistanceFn>()
        .register_inspectable::<DistanceFn>()
        .run();
}

fn setup(mut commands: Commands, rng: Res<RandomNumbers>) {
    let mut map = TileMap(Grid::default([SCREEN_WIDTH, SCREEN_HEIGHT]));
    let mut hm = HeightMap(Grid::default([SCREEN_WIDTH, SCREEN_HEIGHT]));
    let settings = NoiseSettings {
        seed: rng.next_u64(),
        ..Default::default()
    };

    hm.build(&settings);
    map.build(&hm);

    commands.insert_resource(hm);
    commands.insert_resource(map);
    commands.insert_resource(settings);
}

fn render(ctx: Res<BracketContext>, map: Res<TileMap>) {
    for y in 0..SCREEN_HEIGHT {
        for x in 0..SCREEN_WIDTH {
            let tile = &map.0[[x, y]];
            let (glyph, color) = match tile {
                TileType::Water => ('~', RGB::named(BLUE)),
                TileType::Land => ('░', RGB::named(YELLOW)),
                TileType::Mountain => ('^', RGB::named(WHITE)),
            };

            ctx.set(x, y, color, RGB::named(BLACK), to_cp437(glyph));
        }
    }
}

fn regen(
    rng: Res<RandomNumbers>,
    mut map: ResMut<TileMap>,
    mut hm: ResMut<HeightMap>,
    mut settings: ResMut<NoiseSettings>,
    data: Res<Data>,
) {
    if data.is_changed() {
        settings.seed = rng.next_u64();

        hm.build(&data.custom);
        map.build(&hm)
    }
}
