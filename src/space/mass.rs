use bevy::prelude::*;

pub struct MassPlugin {}

impl Plugin for MassPlugin {
    fn build(&self, app: &mut App) {}
}

#[derive(Component)]
pub struct Mass(pub f32);
