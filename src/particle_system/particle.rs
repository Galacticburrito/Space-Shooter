use crate::{color_palette::PalColor, primitive::Primitive};
use bevy::prelude::*;
use serde::Deserialize;

/// `lifetime`: secs
#[derive(Clone, Debug, Deserialize, Reflect)]
pub struct ParticleData {
    pub shape: Primitive,
    pub lifetime: f32,
    pub speed: f32,
    pub color_start_end: (PalColor, PalColor),
    pub size_start_end: (f32, f32),
}

impl ParticleData {
    pub fn new(
        shape: Primitive,
        lifetime: f32,
        speed: f32,
        color_start_end: (PalColor, PalColor),
        size_start_end: (f32, f32),
    ) -> Self {
        ParticleData {
            shape,
            lifetime,
            speed,
            color_start_end,
            size_start_end,
        }
    }
}

#[derive(Component)]
pub struct Particle(pub ParticleData);
