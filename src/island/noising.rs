use super::island_prelude::*;
use crate::{GradientFn, GradientType, NoiseMap};
use noise::Add;

pub struct NoiseSettings {
    pub octaves: usize,
    pub persistence: f64,
    pub lacunarity: f64,

    pub moisture_map_sub: f32,
    pub moisture_map_frequency: f64,
    pub moisture_map_gradient_mult: f32,

    pub height_map_mult: f32,
    pub height_map_frequency: f64,
    pub height_map_gradient_mult: f32,

    pub low: f64,
    pub high: f64,

    pub gradient_fn: Box<dyn GradientFn>,
}

impl Default for NoiseSettings {
    fn default() -> Self {
        Self {
            low: 0.0,
            high: 1.0,
            octaves: 7,
            lacunarity: 2.0,
            persistence: 0.5,
            gradient_fn: Box::new(GradientType::Euclidean2),

            moisture_map_sub: 1.6,
            moisture_map_frequency: 0.02,
            moisture_map_gradient_mult: 0.4,

            height_map_mult: 1.2,
            height_map_frequency: 0.04,
            height_map_gradient_mult: 0.3,
        }
    }
}

//////////////////////////////////////////////////////////////////////////////////////////////

pub struct InternalNoiseMap {
    pub seed: u64,
    pub height_map: HeightMap,
    pub moisture_map: HeightMap,
    pub settings: NoiseSettings,
}

impl Default for InternalNoiseMap {
    fn default() -> Self {
        Self {
            seed: 0,
            settings: NoiseSettings::default(),
            moisture_map: HeightMap::zeros((SCREEN_WIDTH as usize, SCREEN_HEIGHT as usize)),
            height_map: HeightMap::zeros((SCREEN_WIDTH as usize, SCREEN_HEIGHT as usize)),
        }
    }
}

impl InternalNoiseMap {
    pub fn generate_gradient(&self) -> HeightMap {
        ndarray::Array2::from_shape_fn((SCREEN_WIDTH as usize, SCREEN_HEIGHT as usize), |(x, y)| {
            self.settings.gradient_fn.distance(x as f64, y as f64)
        })
    }

    pub fn generate_height_map(&mut self, scale: f64) -> HeightMap {
        let open = noise::OpenSimplex::new(self.seed as u32);
        let perlin = noise::Simplex::new(self.seed as u32);
        let add = Add::new(open, perlin);

        NoiseMap::new(
            self.settings.octaves,
            self.settings.persistence,
            scale,
            self.settings.lacunarity,
            (self.settings.low, self.settings.high),
        )
        .generate(
            (SCREEN_WIDTH as usize, SCREEN_HEIGHT as usize),
            Box::new(add),
        )
    }

    pub fn generate_maps(&mut self) {
        let gradient = self.generate_gradient();
        self.moisture_map = self.generate_height_map(self.settings.moisture_map_frequency);
        self.height_map = self.generate_height_map(self.settings.height_map_frequency);

        for ((x, y), v) in self.height_map.indexed_iter_mut() {
            *v = *v * self.settings.height_map_mult as f64
                - gradient.get((x, y)).unwrap() * self.settings.height_map_gradient_mult as f64
        }

        for ((x, y), v) in self.moisture_map.indexed_iter_mut() {
            *v -= (self.settings.moisture_map_sub as f64 - gradient.get((x, y)).unwrap())
                * self.settings.moisture_map_gradient_mult as f64
        }
    }
}
