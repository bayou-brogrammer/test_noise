use super::bracket_prelude::*;
use sark_grids::Grid;

#[derive(Default, Clone, Reflect)]
#[reflect(Resource)]
pub enum TileType {
    #[default]
    Water,
    Land,
    Mountain,
}

pub fn idx(x: i32, y: i32) -> usize {
    (x + y * SCREEN_WIDTH) as usize
}

#[derive(Default, Clone)]
pub struct TileMap(pub Grid<TileType>);

impl TileMap {
    pub fn build(&mut self, hm: &HeightMap) {
        for y in 0..SCREEN_HEIGHT {
            for x in 0..SCREEN_WIDTH {
                self.0[[x, y]] = if hm.0[[x, y]] < 0.4 {
                    TileType::Water
                } else if hm.0[[x, y]] < 0.6 {
                    TileType::Land
                } else {
                    TileType::Mountain
                };
            }
        }
    }
}

#[derive(Clone)]
pub struct HeightMap(pub Grid<f32>);

impl HeightMap {
    pub fn build(&mut self, settings: &NoiseSettings) {
        let mut noise = FastNoise::seeded(settings.seed);
        noise.set_noise_type(NoiseType::PerlinFractal);
        noise.set_fractal_type(FractalType::FBM);
        noise.set_fractal_gain(settings.gain);
        noise.set_frequency(settings.frequency);
        noise.set_fractal_octaves(settings.octaves);
        noise.set_fractal_lacunarity(settings.lacunarity);

        for y in 0..SCREEN_HEIGHT {
            for x in 0..SCREEN_WIDTH {
                let nx = 2.0 * x as f32 / SCREEN_WIDTH as f32 - 1.0;
                let ny = 2.0 * y as f32 / SCREEN_HEIGHT as f32 - 1.0;

                let a2 = nx.powf(2.0);
                let b2 = ny.powf(2.0);

                let d = match settings.distance_fn {
                    DistanceFn::Square => 1.0 - (1.0 - a2) * (1.0 - b2),
                    DistanceFn::Euclidean => {
                        let sqrt2 = 2_f32.sqrt() - 0.5;
                        f32::min(1.0, (a2 + b2) / sqrt2)
                    }
                };

                let n = ((noise.get_noise(x as f32 / settings.x_div, y as f32 / settings.y_div))
                    + 1.0)
                    / settings.total_div;

                let e = (n + (1.0 - d)) / 2.0;
                self.0[[x, y]] = e;
            }
        }
    }
}
