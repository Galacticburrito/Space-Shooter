use bevy::prelude::*;
pub mod emitter;
pub mod particle;

pub struct ParticleSystemPlugin;

impl Plugin for ParticleSystemPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(emitter::EmitterPlugin {});
    }
}
