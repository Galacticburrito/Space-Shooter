use super::mass::Mass;
use crate::{
    SystemUpdateSet, data_config::global_settings::GlobalSettings, debug, velocity::Velocity,
};
use bevy::prelude::*;

pub struct GravityPlugin {}

impl Plugin for GravityPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (apply_gravity).in_set(SystemUpdateSet::Main));
    }
}

/// affected by gravity
#[derive(Component)]
pub struct Gravitated {}

/// affect gravity
#[derive(Component)]
pub struct GravitySource {}

fn apply_gravity(
    mut param_set: ParamSet<(
        Query<(&Transform, &Mass), With<GravitySource>>, // sources
        Query<(&Transform, &mut Velocity), With<Gravitated>>, // affected
    )>,
    g_settings: Res<GlobalSettings>,
    time: Res<Time>,
) {
    let sources: Vec<(Transform, f32)> = param_set
        .p0()
        .iter()
        .map(|(transform, mass)| (*transform, mass.0))
        .collect();

    for (source_transform, source_mass) in &sources {
        for (affected_transform, mut affected_velocity) in &mut param_set.p1() {
            if source_transform == affected_transform {
                // same object
                continue;
            }

            // compute acceleration of affected (mass cancels out)
            let acceleration = g_settings.gravity_const * (source_mass)
                / source_transform
                    .translation
                    .distance_squared(affected_transform.translation);

            // point acceleration vector to source, then add to affected velocity
            let dir = (source_transform.translation.xy() - affected_transform.translation.xy())
                .normalize_or(Vec2::new(0., 0.));
            affected_velocity.0 += dir.xy() * acceleration * time.delta_secs();
        }
    }
}
