use crate::SystemUpdateSet;
use bevy::prelude::*;

pub struct HeaterPlugin {}

impl Plugin for HeaterPlugin {
    fn build(&self, app: &mut App) {
      app
            .register_type::<Heater>();
    }
}

/// an entity that produces heat at certain rate
#[derive(Component, Reflect)]
pub struct Heater {
    rate: f32,
}

impl Heater {
    pub fn new(rate: f32) -> Self {
        Heater {
            rate,
        }
    }
}

/// entities that can get heated up
#[derive(Component, Reflect)]
pub struct Heat {
    current: f32,
    max: f32,
}

impl Heat {
    pub fn new(max: f32) -> Self {
        Heat {
            current: 0,
            max,
            // bool if flammable, so explodes if on fire?
        }
    }
}

/// marker component for entities on fire
#[derive(Component, Reflect)]
pub struct OnFire {}