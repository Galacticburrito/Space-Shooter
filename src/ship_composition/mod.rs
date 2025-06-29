use bevy::prelude::*;
pub mod bullet;
pub mod engine;
pub mod gun;
pub mod sonar;

pub struct ShipCompositionPlugin {}

impl Plugin for ShipCompositionPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            bullet::BulletPlugin {},
            engine::EnginePlugin {},
            gun::GunPlugin {},
            sonar::SonarPlugin {},
        ));
    }
}
