use bevy::prelude::*;
mod gravity;
pub mod mass;
mod planet;

pub struct SpacePlugin {}

impl Plugin for SpacePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            planet::PlanetPlugin {},
            mass::MassPlugin {},
            gravity::GravityPlugin {},
        ));
    }
}
