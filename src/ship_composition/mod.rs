use bevy::prelude::*;

pub mod engine;
pub mod gun;

pub fn setup(app: &mut App) {
    app.add_plugins((engine::EnginePlugin {}, gun::GunPlugin {}));
}
