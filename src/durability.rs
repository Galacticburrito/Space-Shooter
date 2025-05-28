use crate::SystemUpdateSet;
use bevy::prelude::*;

pub struct DurabilityPlugin {}

impl Plugin for DurabilityPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Durability>();
    }
}

#[derive(Component, Reflect)]
pub struct Durability {
    max: f32,
    current: f32,
    decay_rate: f32,
}

impl Durability {
    pub fn new(max: f32, decay_rate: f32) -> Self {
        Durability {
            max,
            current: max,
            decay_rate,
        }
    }
}
