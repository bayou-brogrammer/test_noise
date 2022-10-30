use super::island_prelude::*;
use noise::NoiseFn;

//////////////////////////////////////////////////////////////////////////////////////////////
// Height Maps
//////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Clone, Debug, Default)]
pub struct NoiseMap {
    pub low: f64,
    pub high: f64,

    pub octaves: usize,
    pub frequency: f64,
    pub lacunarity: f64,
    pub persistence: f64,
}

impl NoiseMap {
    pub fn new(
        octaves: usize,
        persistence: f64,
        frequency: f64,
        lacunarity: f64,
        lh_shape: (f64, f64),
    ) -> Self {
        Self {
            low: lh_shape.0,
            high: lh_shape.1,
            octaves,
            frequency,
            lacunarity,
            persistence,
        }
    }

    pub fn generate(&self, shape: (usize, usize), noise_fn: Box<dyn NoiseFn<f64, 2>>) -> HeightMap {
        ndarray::Array2::from_shape_fn(shape, |(x, y)| {
            self.sum_octaves((x, y), |[x, y]| noise_fn.get([x, y]))
        })
    }

    fn sum_octaves(&self, point: (usize, usize), noise_fn: impl Fn([f64; 2]) -> f64) -> f64 {
        let mut amp = 1.0;
        let mut max_amp = 0.0;
        let mut freq = self.frequency;

        let mut noise = 0.0;
        for _ in 0..self.octaves {
            noise += noise_fn([point.0 as f64 * freq, point.1 as f64 * freq]) * amp;
            max_amp += amp;
            amp *= self.persistence;
            freq *= self.lacunarity;
        }

        (noise / max_amp) * (self.high - self.low) / 2.0 + (self.high + self.low) / 2.0
    }
}
