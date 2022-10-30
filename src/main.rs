mod bracket_noise_fn;
mod island;

pub use bracket_noise_fn::*;
pub use island::*;

mod prelude {
    pub use bevy::prelude::*;
    pub use bracket_bevy::prelude::*;
    pub use bracket_bevy::FontCharType;
}
use clap::{Parser, ValueEnum};
pub use prelude::*;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum Mode {
    /// Run Noise Fn with Bracket-Noise
    BracketNoise,
    /// Generate Island using noise-rs
    Island,
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// What mode to run the program in
    #[arg(value_enum)]
    pub mode: Mode,
}

fn main() {
    let cli = Cli::parse();

    let mut app = App::new();
    app.add_plugins(DefaultPlugins);

    match cli.mode {
        Mode::BracketNoise => bracket_noise_fn::run_bracket_instance(&mut app),
        Mode::Island => island::run_island_gen(&mut app),
    }
}
