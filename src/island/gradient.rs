use super::island_prelude::*;

pub trait GradientFn: Send + Sync {
    fn distance(&self, x: f64, y: f64) -> f64;
}

impl GradientFn for GradientType {
    fn distance(&self, x: f64, y: f64) -> f64 {
        let nx = 2. * x as f64 / SCREEN_WIDTH as f64 - 1.;
        let ny = 2. * y as f64 / SCREEN_HEIGHT as f64 - 1.;

        match self {
            GradientType::SquareBump => 1.0 - (1.0 - nx.powi(2)) * (1.0 - ny.powi(2)),
            GradientType::DistanceSquared => 1.0 - (nx.powi(2) + ny.powi(2)),
            GradientType::Squircle => 1.0 - (nx.powi(4) + ny.powi(4)).sqrt(),
            GradientType::Hyperboloid => 1.0 - (nx.powi(2) + ny.powi(2) + 0.2_f64.powi(2)).sqrt(),
            GradientType::Euclidean2 => {
                f64::min(1.0, (nx.powi(2) + ny.powi(2)) / std::f64::consts::SQRT_2)
            }
        }
    }
}

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub enum GradientType {
    #[default]
    Euclidean2,
    Squircle,
    SquareBump,
    Hyperboloid,
    DistanceSquared,
}
