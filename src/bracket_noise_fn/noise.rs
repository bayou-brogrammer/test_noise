use super::bracket_prelude::*;
use bevy_inspector_egui::Inspectable;

pub fn gen_noise(noise: &FastNoise, nx: f32, ny: f32) -> f32 {
    noise.get_noise(nx, ny) / 2.0 + 0.5
}

#[derive(Debug, Default, Clone, Reflect, Eq, PartialEq)]
#[reflect(Resource)]
pub enum DistanceFn {
    Square,
    #[default]
    Euclidean,
}

impl Inspectable for DistanceFn {
    type Attributes = ();

    fn ui(
        &mut self,
        ui: &mut bevy_inspector_egui::egui::Ui,
        _: Self::Attributes,
        _: &mut bevy_inspector_egui::Context,
    ) -> bool {
        ui.selectable_value(self, DistanceFn::Square, "Square");
        ui.selectable_value(self, DistanceFn::Euclidean, "Euclidean");
        false
    }
}

#[derive(Debug, Reflect)]
#[reflect(Resource)]
pub struct NoiseSettings {
    pub seed: u64,

    pub gain: f32,
    pub octaves: i32,
    pub frequency: f32,
    pub lacunarity: f32,

    pub x_div: f32,
    pub y_div: f32,
    pub total_div: f32,

    pub distance_fn: DistanceFn,
}

impl Default for NoiseSettings {
    fn default() -> Self {
        Self {
            seed: 0,
            gain: 0.2,
            octaves: 8,
            frequency: 4.0,
            lacunarity: 3.0,

            x_div: 50.0,
            y_div: 25.0,
            total_div: 2.0,

            distance_fn: DistanceFn::default(),
        }
    }
}
