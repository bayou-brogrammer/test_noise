use super::island_prelude::*;

#[derive(Debug, Clone, Default, Eq, PartialEq)]
pub enum Biome {
    /// From deepest part of the map -> highest part of the map
    #[default]
    DeepWater,
    Water,
    Sand,
    Dirt,
    Grass,

    LightForest,
    Mountain,
    Snow,
}

pub fn get_biome_color(tt: &Biome) -> (FontCharType, (u8, u8, u8)) {
    match tt {
        Biome::Grass => (to_cp437('"'), GREEN),
        Biome::Dirt => (to_cp437('d'), SADDLEBROWN),
        Biome::Water => (to_cp437('~'), CYAN),
        Biome::DeepWater => (to_cp437('~'), BLUE),
        Biome::Sand => (to_cp437('░'), YELLOW),
        Biome::LightForest => (to_cp437('¶'), LIGHTGREEN),
        Biome::Mountain => (to_cp437('^'), SLATEGRAY),
        Biome::Snow => (to_cp437('s'), WHITE),
    }
}

pub fn get_biome_at(height: f64, moisture: f64) -> Biome {
    match (height, moisture) {
        (a, _) if a < 0.35 => Biome::DeepWater,
        (a, _) if a < 0.42 => Biome::Water,
        (a, b) if a < 0.46 && b < 0.57 => Biome::Sand,
        (a, b) if a < 0.47 && b >= 0.6 => Biome::Dirt,
        (a, b) if a > 0.54 && b < 0.43 && a < 0.62 => Biome::Grass,
        (a, b) if a < 0.62 && b >= 0.49 => Biome::LightForest,
        (a, b) if a >= 0.68 && b >= 0.10 => Biome::Mountain,
        (a, _) if a >= 0.79 => Biome::Snow,
        _ => Biome::LightForest,
    }
}
